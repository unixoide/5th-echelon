# UserAccountManagementService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| ? | [UserAccountManagementProtocol](#useraccountmanagementprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## UserAccountManagementProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func LookupSceNpIds(pids: std_list<uint32>) -> (npids: std_map<uint32, qBuffer>)
```

</td></tr>
<tr><td>2</td><td>

```swift
func LookupPrincipalIDs(firstPartyIds: std_list<string>, platformId: uint32) -> (pids: std_map<string, uint32>)
```

</td></tr>
<tr><td>3</td><td>

```swift
func LookupFirstPartyIds(pids: std_list<uint32>, platformId: uint32) -> (firstPartyIds: std_map<uint32, string>)
```

</td></tr>
<tr><td>4</td><td>

```swift
func UserHasPlayed(FirstPartyIds: std_list<string>, platformId: uint32) -> (UserPresence: std_map<string, bool>)
```

</td></tr>
<tr><td>5</td><td>

```swift
func IsUserPlaying(firstPartyIds: std_list<string>, platformId: uint32) -> (UserPresence: std_map<string, bool>)
```

</td></tr>
<tr><td>6</td><td>

```swift
func updateSonyAccountInfo(ticketData: qBuffer, ticketSize: uint32) -> ()
```

</td></tr>
<tr><td>7</td><td>

```swift
func LookupUsernames(pids: std_list<uint32>, platformId: uint32) -> (UserNames: std_map<uint32, string>)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
