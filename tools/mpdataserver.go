package main

import (
	"bytes"
	"crypto/ecdsa"
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"crypto/x509/pkix"
	"encoding/pem"
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"math/big"
	"net/http"
	"os"
	"time"
)

func index(w http.ResponseWriter, r *http.Request) {
	io.WriteString(w, `[Echelon.EPlayerPawn]
SpyWeaponEffectDurationLookUpTable=3.0f;
SpyWeaponEffectDurationLookUpTable=2.0f;
SpyWeaponEffectDurationLookUpTable=1.0f;

[Echelon.GI_Adv_AntiGrenadeSystem]
m_AdversarialInventoryLimit=2

[MPArmorHelmetTech]
m_fBMSRefreshDelayMultiplier=0.8

[EXPLOSION_STICKYCAM_ADV]
GroundZeroRadius=280
GroundZeroZoneDamage=150
BlastZoneDamage=80
NPCGroundZeroRadius=280
NPCBlastRadius=600
NPCGroundZeroZoneDamage=150
NPCBlastZoneDamage=80

[EXPLOSION_PROXMINE_ADV]
GroundZeroZoneDamage=150

[EXPLOSION_GRENADE_ADV]
GroundZeroZoneDamage=150

[EXPLOSION_ADVDRONE]
GroundZeroZoneDamage=150
`)
}

func publicKey(priv interface{}) interface{} {
	switch k := priv.(type) {
	case *rsa.PrivateKey:
		return &k.PublicKey
	case *ecdsa.PrivateKey:
		return &k.PublicKey
	default:
		return nil
	}
}

func pemBlockForKey(priv interface{}) *pem.Block {
	switch k := priv.(type) {
	case *rsa.PrivateKey:
		return &pem.Block{Type: "RSA PRIVATE KEY", Bytes: x509.MarshalPKCS1PrivateKey(k)}
	case *ecdsa.PrivateKey:
		b, err := x509.MarshalECPrivateKey(k)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Unable to marshal ECDSA private key: %v", err)
			os.Exit(2)
		}
		return &pem.Block{Type: "EC PRIVATE KEY", Bytes: b}
	default:
		return nil
	}
}

func createCA() (*x509.Certificate, *rsa.PrivateKey) {
	priv, err := rsa.GenerateKey(rand.Reader, 4096)
	if err != nil {
		log.Fatal(err)
	}
	template := x509.Certificate{
		SerialNumber: big.NewInt(1),
		Subject: pkix.Name{
			Organization: []string{"5th Echelon"},
		},
		NotBefore: time.Now(),
		NotAfter:  time.Now().Add(time.Hour * 24 * 356 * 10),

		KeyUsage:              x509.KeyUsageKeyEncipherment | x509.KeyUsageDigitalSignature | x509.KeyUsageCertSign,
		BasicConstraintsValid: true,
		IsCA:                  true,
	}
	derBytes, err := x509.CreateCertificate(rand.Reader, &template, &template, publicKey(priv), priv)
	if err != nil {
		log.Fatalf("Failed to create certificate: %s", err)
	}
	out := &bytes.Buffer{}
	pem.Encode(out, &pem.Block{Type: "CERTIFICATE", Bytes: derBytes})
	ioutil.WriteFile("ca.pem", out.Bytes(), 0644)
	out.Reset()
	pem.Encode(out, pemBlockForKey(priv))
	ioutil.WriteFile("ca.key", out.Bytes(), 0644)
	return &template, priv
}

func createCert(ca *x509.Certificate, caKey *rsa.PrivateKey) {
	priv, err := rsa.GenerateKey(rand.Reader, 4096)
	if err != nil {
		log.Fatal(err)
	}
	template := x509.Certificate{
		SerialNumber: big.NewInt(1),
		Subject: pkix.Name{
			Organization: []string{"5th Echelon"},
			CommonName:   "sc6_pc_lnch_b.s3.amazonaws.com",
		},
		NotBefore: time.Now(),
		NotAfter:  time.Now().Add(time.Hour * 24 * 356 * 10),

		KeyUsage:              x509.KeyUsageKeyEncipherment | x509.KeyUsageDigitalSignature,
		ExtKeyUsage:           []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		BasicConstraintsValid: true,
	}
	derBytes, err := x509.CreateCertificate(rand.Reader, &template, ca, publicKey(priv), caKey)
	if err != nil {
		log.Fatalf("Failed to create certificate: %s", err)
	}
	out := &bytes.Buffer{}
	pem.Encode(out, &pem.Block{Type: "CERTIFICATE", Bytes: derBytes})
	ioutil.WriteFile("cert.pem", out.Bytes(), 0644)
	out.Reset()
	pem.Encode(out, pemBlockForKey(priv))
	ioutil.WriteFile("key.pem", out.Bytes(), 0644)
}

func main() {
	if _, err := os.Stat("cert.pem"); err != nil {
		createCert(createCA())
	}
	http.HandleFunc("/", index)
	log.Fatal(http.ListenAndServeTLS("127.0.0.1:443", "cert.pem", "key.pem", http.DefaultServeMux))
}
