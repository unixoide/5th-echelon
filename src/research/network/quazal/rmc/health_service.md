# HealthService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 18 | [HealthProtocol](#healthprotocol) |
| 19 | [MonitoringProtocol](#monitoringprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## HealthProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func PingDaemon() -> (retval: bool)
```

</td></tr>
<tr><td>2</td><td>

```swift
func PingDatabase() -> (retval: bool)
```

</td></tr>
<tr><td>3</td><td>

```swift
func RunSanityCheck() -> (retval: bool)
```

</td></tr>
<tr><td>4</td><td>

```swift
func FixSanityErrors() -> (retval: bool)
```

</td></tr>
</tbody></table>
## MonitoringProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func PingDaemon() -> (retval: bool)
```

</td></tr>
<tr><td>2</td><td>

```swift
func GetClusterMembers() -> (strValues: std_list<string>)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
