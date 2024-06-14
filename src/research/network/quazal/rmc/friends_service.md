# FriendsService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 20 | [FriendsProtocol](#friendsprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## FriendsProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func AddFriend(uiPlayer: uint32, uiDetails: uint32, strMessage: string) -> (retval: bool)
```

</td></tr>
<tr><td>2</td><td>

```swift
func AddFriendByName(strPlayerName: string, uiDetails: uint32, strMessage: string) -> (retval: bool)
```

</td></tr>
<tr><td>3</td><td>

```swift
func AddFriendWithDetails(uiPlayer: uint32, uiDetails: uint32, strMessage: string) -> (relationshipData: [[RelationshipData]]((relationshipdata)))
```

</td></tr>
<tr><td>4</td><td>

```swift
func AddFriendByNameWithDetails(strPlayerName: string, uiDetails: uint32, strMessage: string) -> (relationshipData: [[RelationshipData]]((relationshipdata)))
```

</td></tr>
<tr><td>5</td><td>

```swift
func AcceptFriendship(uiPlayer: uint32) -> (retval: bool)
```

</td></tr>
<tr><td>6</td><td>

```swift
func DeclineFriendship(uiPlayer: uint32) -> (retval: bool)
```

</td></tr>
<tr><td>7</td><td>

```swift
func BlackList(uiPlayer: uint32, uiDetails: uint32) -> (retval: bool)
```

</td></tr>
<tr><td>8</td><td>

```swift
func BlackListByName(strPlayerName: string, uiDetails: uint32) -> (retval: bool)
```

</td></tr>
<tr><td>9</td><td>

```swift
func ClearRelationship(uiPlayer: uint32) -> (retval: bool)
```

</td></tr>
<tr><td>10</td><td>

```swift
func UpdateDetails(uiPlayer: uint32, uiDetails: uint32) -> (retval: bool)
```

</td></tr>
<tr><td>11</td><td>

```swift
func GetList(byRelationship: byte, bReversed: bool) -> (lstFriendsList: std_list<uint32>)
```

</td></tr>
<tr><td>12</td><td>

```swift
func GetDetailedList(byRelationship: byte, bReversed: bool) -> (lstFriendsList: std_list<[[FriendData]]((frienddata))>)
```

</td></tr>
<tr><td>13</td><td>

```swift
func GetRelationships(resultRange: [[ResultRange]]((resultrange))) -> (uiTotalCount: uint32, lstRelationshipsList: std_list<[[RelationshipData]]((relationshipdata))>)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
