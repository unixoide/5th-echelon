# LadderHelperService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 107 | [LadderHelperProtocol](#ladderhelperprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## LadderHelperProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func GetUnixUTC() -> (time: uint32)
```

</td></tr>
<tr><td>2</td><td>

```swift
func AreLaddersAvailableInCountry() -> (allowed: bool)
```

</td></tr>
<tr><td>3</td><td>

```swift
func CheckLadderIsRunning(startTime: uint32, endTime: uint32) -> (running: bool)
```

</td></tr>
<tr><td>4</td><td>

```swift
func ClearLadderLeaderboard(statSet: int32) -> (success: bool)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
