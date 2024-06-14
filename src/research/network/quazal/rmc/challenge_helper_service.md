# ChallengeHelperService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 105 | [ChallengeHelperProtocol](#challengehelperprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## ChallengeHelperProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func GenerateMyFriendChallenges(friendPIDs: qlist<uint32>) -> (result: qlist<[[FriendChallenge]]((friendchallenge))>)
```

</td></tr>
<tr><td>2</td><td>

```swift
func GenerateFriendChallenges(targetPID: uint32, friendPIDs: qlist<uint32>) -> (result: qlist<[[FriendChallenge]]((friendchallenge))>)
```

</td></tr>
<tr><td>3</td><td>

```swift
func GetOnlineChallenges() -> (onlineChallenges: qlist<[[OnlineChallenge]]((onlinechallenge))>)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
