# SimpleAuthentication

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 16 | [SimpleAuthenticationProtocol](#simpleauthenticationprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## SimpleAuthenticationProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func Authenticate(strUserName: string) -> (retval: qresult, pidPrincipal: uint32, pConnectionData: [[RVConnectionData]]((rvconnectiondata)), strReturnMsg: string)
```

</td></tr>
<tr><td>2</td><td>

```swift
func LoginWithToken(strToken: string) -> (retval: qresult, pidPrincipal: uint32, pConnectionData: [[RVConnectionData]]((rvconnectiondata)), strReturnMsg: string)
```

</td></tr>
<tr><td>3</td><td>

```swift
func LoginWithTokenEx(strToken: string, oAnyData: any<[[Data]]((data)), string>) -> (retval: qresult, pidPrincipal: uint32, pConnectionData: [[RVConnectionData]]((rvconnectiondata)), strReturnMsg: string)
```

</td></tr>
<tr><td>4</td><td>

```swift
func Login(strUsername: string, strPassword: string) -> (retval: qresult, pidPrincipal: uint32, pConnectionData: [[RVConnectionData]]((rvconnectiondata)), strReturnMsg: string)
```

</td></tr>
<tr><td>5</td><td>

```swift
func LoginWithSubAccount(loginData: any<[[Data]]((data)), string>) -> (retval: qresult, pidPrincipal: uint32, pConnectionData: [[RVConnectionData]]((rvconnectiondata)), strReturnMsg: string)
```

</td></tr>
<tr><td>6</td><td>

```swift
func Register(vecMyURLs: std_list<stationurl>) -> (retval: qresult, pidConnectionID: uint32, urlPublic: stationurl)
```

</td></tr>
<tr><td>7</td><td>

```swift
func RegisterEx(vecMyURLs: std_list<stationurl>, hCustomData: any<[[Data]]((data)), string>) -> (retval: qresult, pidConnectionID: uint32, urlPublic: stationurl)
```

</td></tr>
<tr><td>8</td><td>

```swift
func LoginWithTokenCafe(strNintendoToken: string) -> (retval: qresult, pidPrincipal: uint32, pConnectionData: [[RVConnectionData]]((rvconnectiondata)), strReturnMsg: string)
```

</td></tr>
<tr><td>9</td><td>

```swift
func LoginWithTokenCafeEx(strNintendoToken: string, oAnyData: any<[[Data]]((data)), string>) -> (retval: qresult, pidPrincipal: uint32, pConnectionData: [[RVConnectionData]]((rvconnectiondata)), strReturnMsg: string)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
