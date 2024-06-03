# Splinter Cell: Blacklist

## Network

- [Quazal](./quazal.md)
- Authentication key: `yl4NG7qZ`
- Encryption key: `CD&ML`

## Logic

- Primarily done through state machines
  - Each state machine has multiple states, identified by IDs (generated from the state name)
  - State transitions are either happening  directly or via Goals

## Inviting others

- Invites are done via UPlay ([UPLAY_Friends_InviteToGame](../../hooks/src/uplay_r1_loader/friends.rs))

### Questions
- Accepting invites as well?
- How does uplay launch the game into join mode? [Example](https://www.youtube.com/watch?v=d45CYK_LuYA)



## custom overlay

- Direct3D uses COM interfaces
 - Calling convention: stdcall with this as first parameter


# PC

## WW

```
https://onlineconfigservice.ubi.com/OnlineConfigService.svc/GetOnlineConfig?onlineConfigID=967fad701a3648d8bf099f07207f4a73&target=client
```
```json
[
  {
    "Name": "punch_DetectUrls",
    "Values": [
      "lb-ne1z-prod-mpe-detect01.ubisoft.com:11000",
      "lb-ne1z-prod-mpe-detect02.ubisoft.com:11000"
    ]
  },
  {
    "Name": "SandboxUrl",
    "Values": [
      "prudp:/address=lb-rdv-as-prod01.ubisoft.com;port=21126"
    ]
  },
  {
    "Name": "SandboxUrlWS",
    "Values": [
      "ne1-z3-as-rdv03.ubisoft.com:21125"
    ]
  },
  {
    "Name": "uplay_DownloadServiceUrl",
    "Values": [
      "https://secure.ubi.com/UplayServices/UplayFacade/DownloadServicesRESTXML.svc/REST/XML/?url="
    ]
  },
  {
    "Name": "uplay_DynContentBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/"
    ]
  },
  {
    "Name": "uplay_DynContentSecureBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/"
    ]
  },
  {
    "Name": "uplay_LinkappBaseUrl",
    "Values": [
      " http://static8.ubi.com/private/Uplay/Packages/linkapp/3.0.0-rc/"
    ]
  },
  {
    "Name": "uplay_PackageBaseUrl",
    "Values": [
      "http://static8.ubi.com/private/Uplay/Packages/1.5-Share-rc/"
    ]
  },
  {
    "Name": "uplay_WebServiceBaseUrl",
    "Values": [
      "https://secure.ubi.com/UplayServices/UplayFacade/ProfileServicesFacadeRESTXML.svc/REST/"
    ]
  }
]
```

```
https://onlineconfigservice.ubi.com/OnlineConfigService.svc/GetOnlineConfig?onlineConfigID=967fad701a3648d8bf099f07207f4a73&target=server
```
```json
[
  {
    "Name": "name",
    "Values": [
      "SC6_PC_LNCH_B (WW)"
    ]
  },
  {
    "Name": "product_id",
    "Values": [
      "a1b16676-daa4-44f9-82bd-1c284017024c"
    ]
  }
]
```

# PS3

Authentication key: `lON6yKGp`

## WW

```
https://onlineconfigservice.ubi.com/OnlineConfigService.svc/GetOnlineConfig?onlineConfigID=acd2bb86618441a7b2af9b4b952c9612&target=client
```

```json
[
  {
    "Name": "punch_DetectUrls",
    "Values": [
      "lb-ne1z-prod-mpe-detect01.ubisoft.com:11000",
      "lb-ne1z-prod-mpe-detect02.ubisoft.com:11000"
    ]
  },
  {
    "Name": "SandboxUrlPS3",
    "Values": [
      "prudp:/address=lb-rdv-as-prod01.ubisoft.com;port=21121;serviceid=UPxxxx-MYGAME"
    ]
  },
  {
    "Name": "SandboxUrlWS",
    "Values": [
      "ne1-z3-as-rdv03.ubisoft.com:21120"
    ]
  },
  {
    "Name": "uplay_DownloadServiceUrl",
    "Values": [
      "https://wsuplay.ubi.com/UplayServices/UplayFacade/DownloadServicesRESTXML.svc/REST/XML/?url="
    ]
  },
  {
    "Name": "uplay_DynContentBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/"
    ]
  },
  {
    "Name": "uplay_DynContentSecureBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/"
    ]
  },
  {
    "Name": "uplay_LinkappBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/private/Uplay/Packages/linkapp/3.0.0-rc/"
    ]
  },
  {
    "Name": "uplay_MovieBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/"
    ]
  },
  {
    "Name": "uplay_PackageBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/Packages/1.5-Share-rc/"
    ]
  },
  {
    "Name": "uplay_WebServiceBaseUrl",
    "Values": [
      "https://wsuplay.ubi.com/UplayServices/UplayFacade/ProfileServicesFacadeRESTXML.svc/REST/"
    ]
  }
]
```

```
https://onlineconfigservice.ubi.com/OnlineConfigService.svc/GetOnlineConfig?onlineConfigID=acd2bb86618441a7b2af9b4b952c9612&target=server
```
```json
[
  {
    "Name": "name",
    "Values": [
      "SC6_PS3_LNCH_A (WW)"
    ]
  },
  {
    "Name": "product_id",
    "Values": [
      "95d7a4af-cc75-4d6a-9da7-6c06b3956f7b"
    ]
  }
]
```

## Japan

```
https://onlineconfigservice.ubi.com/OnlineConfigService.svc/GetOnlineConfig?onlineConfigID=e2225b01a01a45609daac8160ac36318&target=client
```
```json
[
  {
    "Name": "punch_DetectUrls",
    "Values": [
      "lb-ne1z-prod-mpe-detect01.ubisoft.com:11000",
      "lb-ne1z-prod-mpe-detect02.ubisoft.com:11000"
    ]
  },
  {
    "Name": "SandboxUrlPS3",
    "Values": [
      "prudp:/address=mdc-mm-rdv66.ubisoft.com;port=21180;serviceid=UPxxxx-MYGAME"
    ]
  },
  {
    "Name": "SandboxUrlWS",
    "Values": [
      "mdc-mm-rdv66.ubisoft.com:21180"
    ]
  },
  {
    "Name": "uplay_DownloadServiceUrl",
    "Values": [
      "https://secure.ubi.com/UplayServices/UplayFacade/DownloadServicesRESTXML.svc/REST/XML/?url="
    ]
  },
  {
    "Name": "uplay_DynContentBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/"
    ]
  },
  {
    "Name": "uplay_DynContentSecureBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/"
    ]
  },
  {
    "Name": "uplay_LinkappBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/Packages/linkapp/3.0.0-jp-rc/"
    ]
  },
  {
    "Name": "uplay_MovieBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/"
    ]
  },
  {
    "Name": "uplay_PackageBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/Packages/1.5-Share-jp-rc/"
    ]
  },
  {
    "Name": "uplay_WebServiceBaseUrl",
    "Values": [
      "https://secure.ubi.com/UplayServices/UplayFacade/ProfileServicesFacadeRESTXML.svc/REST/"
    ]
  }
]
```

```
https://onlineconfigservice.ubi.com/OnlineConfigService.svc/GetOnlineConfig?onlineConfigID=e2225b01a01a45609daac8160ac36318&target=server
```
```json
[
  {
    "Name": "name",
    "Values": [
      "SC6_PS3_LNCH_A (JPN)"
    ]
  },
  {
    "Name": "product_id",
    "Values": [
      "95d7a4af-cc75-4d6a-9da7-6c06b3956f7b"
    ]
  }
]
```

# XBOX

Authentication key: `7vPfyD1s`

## WW

```
https://onlineconfigservice.ubi.com/OnlineConfigService.svc/GetOnlineConfig?onlineConfigID=965c0932e166453fa2e42fbda3ee0873&target=client
```
```json
[
  {
    "Name": "punch_DetectUrls",
    "Values": [
      "lb-prod-mm-detect01.ubisoft.com:11020",
      "lb-prod-mm-detect02.ubisoft.com:11020"
    ]
  },
  {
    "Name": "SandboxUrlWS",
    "Values": [
      "ne1-z2-x360-01.ubisoft.com:21035"
    ]
  },
  {
    "Name": "SandboxUrlX360",
    "Values": [
      "prudp:/servername=UBILSP1;port=9475;serviceid=1431504879"
    ]
  },
  {
    "Name": "StorageLspPort",
    "Values": [
      "1082"
    ]
  },
  {
    "Name": "StorageLspServerName",
    "Values": [
      "UBILSP1"
    ]
  },
  {
    "Name": "StorageLspServiceID",
    "Values": [
      "0x555307EF"
    ]
  },
  {
    "Name": "uplay_DownloadServiceUrl",
    "Values": [
      "http://secure.ubi.com/UplayServices/UplayFacade/DownloadServicesRESTXML.svc/REST/XML/?url="
    ]
  },
  {
    "Name": "uplay_DynContentBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/"
    ]
  },
  {
    "Name": "uplay_DynContentSecureBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/"
    ]
  },
  {
    "Name": "uplay_LinkappBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/Packages/linkapp/3.0.0-rc/"
    ]
  },
  {
    "Name": "uplay_MovieBaseUrl",
    "Values": [
      "https://static8.cdn.ubi.com/u/Uplay/"
    ]
  },
  {
    "Name": "uplay_PackageBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/Packages/1.5-Share-rc/"
    ]
  },
  {
    "Name": "uplay_ServiceLspPort",
    "Values": [
      "1081"
    ]
  },
  {
    "Name": "uplay_serviceLSPServerName",
    "Values": [
      "UBILSP1"
    ]
  },
  {
    "Name": "uplay_serviceLspServiceID",
    "Values": [
      "0x555307EF"
    ]
  },
  {
    "Name": "uplay_WebServiceBaseUrl",
    "Values": [
      "http://secure.ubi.com/UplayServices/UplayFacade/ProfileServicesFacadeRESTXML.svc/REST/"
    ]
  }
]
```

```
https://onlineconfigservice.ubi.com/OnlineConfigService.svc/GetOnlineConfig?onlineConfigID=965c0932e166453fa2e42fbda3ee0873&target=server
```
```json
[
  {
    "Name": "name",
    "Values": [
      "SC6_X360_ LNCH _A (WW)"
    ]
  },
  {
    "Name": "product_id",
    "Values": [
      "41f55224-4682-4b58-991a-811958280fcc"
    ]
  }
]
```

## Japan

```
https://onlineconfigservice.ubi.com/OnlineConfigService.svc/GetOnlineConfig?onlineConfigID=1d74ff786559437aad49d38a7f6c6ddd&target=client
```
```json
[
  {
    "Name": "punch_DetectUrls",
    "Values": [
      "lb-prod-mm-detect01.ubisoft.com:11020",
      "lb-prod-mm-detect02.ubisoft.com:11020"
    ]
  },
  {
    "Name": "SandboxUrlWS",
    "Values": [
      "ne1-z2-x360-01.ubisoft.com:21035"
    ]
  },
  {
    "Name": "SandboxUrlX360",
    "Values": [
      "prudp:/servername=UBILSP1;port=9475;serviceid=1431504879"
    ]
  },
  {
    "Name": "StorageLspPort",
    "Values": [
      "1082"
    ]
  },
  {
    "Name": "StorageLspServerName",
    "Values": [
      "UBILSP1"
    ]
  },
  {
    "Name": "StorageLspServiceID",
    "Values": [
      "0x555307EF"
    ]
  },
  {
    "Name": "uplay_DownloadServiceUrl",
    "Values": [
      "http://secure.ubi.com/UplayServices/UplayFacade/DownloadServicesRESTXML.svc/REST/XML/?url="
    ]
  },
  {
    "Name": "uplay_DynContentBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/"
    ]
  },
  {
    "Name": "uplay_DynContentSecureBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/"
    ]
  },
  {
    "Name": "uplay_LinkappBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/Packages/linkapp/3.0.0-jp-rc/"
    ]
  },
  {
    "Name": "uplay_MovieBaseUrl",
    "Values": [
      "https://static8.cdn.ubi.com/u/Uplay/"
    ]
  },
  {
    "Name": "uplay_PackageBaseUrl",
    "Values": [
      "http://static8.cdn.ubi.com/u/Uplay/Packages/1.5-Share-jp-rc/"
    ]
  },
  {
    "Name": "uplay_ServiceLspPort",
    "Values": [
      "1081"
    ]
  },
  {
    "Name": "uplay_serviceLSPServerName",
    "Values": [
      "UBILSP1"
    ]
  },
  {
    "Name": "uplay_serviceLspServiceID",
    "Values": [
      "0x555307EF"
    ]
  },
  {
    "Name": "uplay_WebServiceBaseUrl",
    "Values": [
      "http://secure.ubi.com/UplayServices/UplayFacade/ProfileServicesFacadeRESTXML.svc/REST/"
    ]
  }
]
```

```
https://onlineconfigservice.ubi.com/OnlineConfigService.svc/GetOnlineConfig?onlineConfigID=1d74ff786559437aad49d38a7f6c6ddd&target=server
```
```json
[
  {
    "Name": "name",
    "Values": [
      "SC6_X360_ LNCH _A (JPN)"
    ]
  },
  {
    "Name": "product_id",
    "Values": [
      "41f55224-4682-4b58-991a-811958280fcc"
    ]
  }
]
```