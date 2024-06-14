# UserStorageAdmin

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| ? | [UserStorageAdminProtocol](#userstorageadminprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## UserStorageAdminProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func GetContentsToModerate(typeID: uint32, offset: uint32, size: uint32) -> (contents: qlist<[[UserContent]]((usercontent))>, totalResults: uint32)
```

</td></tr>
<tr><td>2</td><td>

```swift
func FlagContentAsVerified(contentKey: [[UserContentKey]]((usercontentkey))) -> ()
```

</td></tr>
<tr><td>3</td><td>

```swift
func BanContent(contentKey: [[UserContentKey]]((usercontentkey))) -> ()
```

</td></tr>
<tr><td>4</td><td>

```swift
func BanUser(pid: uint32, reason: string, banContents: bool, expireDate: datetime) -> ()
```

</td></tr>
<tr><td>5</td><td>

```swift
func BanUserFromContentType(typeID: uint32, pid: uint32, reason: string, banContents: bool, expireDate: datetime) -> ()
```

</td></tr>
<tr><td>6</td><td>

```swift
func UnbanUser(pid: uint32) -> ()
```

</td></tr>
<tr><td>7</td><td>

```swift
func UnbanUserFromContentType(typeID: uint32, pid: uint32) -> ()
```

</td></tr>
<tr><td>8</td><td>

```swift
func GetContentsToModerateWithThreshold(typeID: uint32, threshold: uint32, offset: uint32, size: uint32) -> (contents: qlist<[[UserContent]]((usercontent))>, totalResults: uint32)
```

</td></tr>
<tr><td>9</td><td>

```swift
func UpdateMetaData(contentKey: [[UserContentKey]]((usercontentkey)), properties: qlist<[[ContentProperty]]((contentproperty))>) -> ()
```

</td></tr>
<tr><td>10</td><td>

```swift
func UpdateContentDB(contentKey: [[UserContentKey]]((usercontentkey)), properties: qlist<[[ContentProperty]]((contentproperty))>, data: string) -> ()
```

</td></tr>
<tr><td>11</td><td>

```swift
func UpdateContentAndGetUploadInfo(contentKey: [[UserContentKey]]((usercontentkey)), properties: qlist<[[ContentProperty]]((contentproperty))>, size: uint32) -> (uploadInfo: [[UserContentURL]]((usercontenturl)), pendingID: uint64, headers: qvector<string>)
```

</td></tr>
<tr><td>12</td><td>

```swift
func DeleteContent(contentKey: [[UserContentKey]]((usercontentkey))) -> ()
```

</td></tr>
<tr><td>13</td><td>

```swift
func BrowseContents(typeID: uint32, offset: uint32, size: uint32) -> (contents: qlist<[[AdminContent]]((admincontent))>, totalResults: uint32)
```

</td></tr>
<tr><td>14</td><td>

```swift
func IsUserbanned(typeID: uint32, pid: uint32) -> (banned: bool, reason: string)
```

</td></tr>
<tr><td>15</td><td>

```swift
func GetBannedUsers(typeID: uint32, offset: uint32, size: uint32) -> (bannedUsers: qlist<[[BannedUser]]((banneduser))>, totalBannedUsers: uint32)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
