# NATTraversal

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| ? | [NATTraversalProtocol](#nattraversalprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## NATTraversalProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func RequestProbeInitiation(urlTargetList: qlist<stationurl>) -> ()
```

</td></tr>
<tr><td>2</td><td>

```swift
func InitiateProbe(urlStationToProbe: stationurl) -> ()
```

</td></tr>
<tr><td>3</td><td>

```swift
func RequestProbeInitiationExt(urlTargetList: qlist<stationurl>, urlStationToProbe: stationurl) -> ()
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
