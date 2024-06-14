# ClanHelperService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 106 | [ClanHelperProtocol](#clanhelperprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## ClanHelperProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func SetClanInfo(newInfo: [[ClanInfo]]((claninfo))) -> ()
```

</td></tr>
<tr><td>2</td><td>

```swift
func AddPIDToCLID(targetPID: uint32, CLID: uint32) -> ()
```

</td></tr>
<tr><td>3</td><td>

```swift
func RemoveMemberByPID(targetPID: uint32) -> ()
```

</td></tr>
<tr><td>4</td><td>

```swift
func DisbandEntireCLID(targetCLID: uint32) -> ()
```

</td></tr>
<tr><td>5</td><td>

```swift
func GetClanInfoByPID(targetPID: uint32) -> (clanInfo: [[ClanInfo]]((claninfo)))
```

</td></tr>
<tr><td>6</td><td>

```swift
func GetClanInfoByCLID(targetCLID: uint32) -> (clanInfo: [[ClanInfo]]((claninfo)))
```

</td></tr>
<tr><td>7</td><td>

```swift
func GetMemberListByPID(targetPID: uint32) -> (members: qlist<uint32>)
```

</td></tr>
<tr><td>8</td><td>

```swift
func GetMemberListByCLID(targetCLID: uint32) -> (members: qlist<uint32>)
```

</td></tr>
<tr><td>9</td><td>

```swift
func GenerateClanChallenges(targetPID: uint32) -> (result: qlist<[[FriendChallenge]]((friendchallenge))>)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
