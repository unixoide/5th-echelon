# AuthenticationFoundation

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 10 | [TicketGrantingProtocol](#ticketgrantingprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## TicketGrantingProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func Login(strUserName: string) -> (retval: qresult, pidPrincipal: uint32, pbufResponse: buffer, pConnectionData: [[RVConnectionData]]((rvconnectiondata)), strReturnMsg: string)
```

</td></tr>
<tr><td>2</td><td>

```swift
func LoginEx(strUserName: string, oExtraData: any<[[Data]]((data)), string>) -> (retval: qresult, pidPrincipal: uint32, pbufResponse: buffer, pConnectionData: [[RVConnectionData]]((rvconnectiondata)), strReturnMsg: string)
```

</td></tr>
<tr><td>3</td><td>

```swift
func RequestTicket(idSource: uint32, idTarget: uint32) -> (retval: qresult, bufResponse: buffer)
```

</td></tr>
<tr><td>4</td><td>

```swift
func GetPID(strUserName: string) -> (retval: uint32)
```

</td></tr>
<tr><td>5</td><td>

```swift
func GetName(id: uint32) -> (retval: string)
```

</td></tr>
<tr><td>6</td><td>

```swift
func LoginWithContext(loginData: any<[[Data]]((data)), string>) -> (retval: qresult, pidPrincipal: uint32, pbufResponse: buffer, pConnectionData: [[RVConnectionData]]((rvconnectiondata)))
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
