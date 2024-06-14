# SecureConnectionService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 11 | [SecureConnectionProtocol](#secureconnectionprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## SecureConnectionProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func Register(vecMyURLs: std_list<stationurl>) -> (retval: qresult, pidConnectionID: uint32, urlPublic: stationurl)
```

</td></tr>
<tr><td>2</td><td>

```swift
func RequestConnectionData(cidTarget: uint32, pidTarget: uint32) -> (retval: bool, pvecConnectionsData: std_list<[[ConnectionData]]((connectiondata))>)
```

</td></tr>
<tr><td>3</td><td>

```swift
func RequestURLs(cidTarget: uint32, pidTarget: uint32) -> (retval: bool, plstURLs: std_list<stationurl>)
```

</td></tr>
<tr><td>4</td><td>

```swift
func RegisterEx(vecMyURLs: std_list<stationurl>, hCustomData: any<[[Data]]((data)), string>) -> (retval: qresult, pidConnectionID: uint32, urlPublic: stationurl)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
