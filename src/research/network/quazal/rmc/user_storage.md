# UserStorage

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 53 | [UserStorageProtocol](#userstorageprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## UserStorageProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func SearchContents(query: [[UserStorageQuery]]((userstoragequery))) -> (searchResults: qlist<[[UserContent]]((usercontent))>)
```

</td></tr>
<tr><td>2</td><td>

```swift
func SearchContentsWithTotal(query: [[UserStorageQuery]]((userstoragequery))) -> (searchResults: qlist<[[UserContent]]((usercontent))>, totalResults: uint32)
```

</td></tr>
<tr><td>3</td><td>

```swift
func DeleteContent(contentKey: [[UserContentKey]]((usercontentkey))) -> ()
```

</td></tr>
<tr><td>4</td><td>

```swift
func SaveMetaData(properties: qlist<[[ContentProperty]]((contentproperty))>) -> ()
```

</td></tr>
<tr><td>5</td><td>

```swift
func SaveContentDB(properties: qlist<[[ContentProperty]]((contentproperty))>, data: buffer) -> ()
```

</td></tr>
<tr><td>6</td><td>

```swift
func SaveContentAndGetUploadInfo(properties: qlist<[[ContentProperty]]((contentproperty))>, size: uint32, contentKey: [[UserContentKey]]((usercontentkey))) -> (uploadInfo: [[UserContentURL]]((usercontenturl)), pendingID: uint64, headers: qvector<string>)
```

</td></tr>
<tr><td>7</td><td>

```swift
func UploadEnd(pendingID: uint64, result: bool) -> ()
```

</td></tr>
<tr><td>8</td><td>

```swift
func GetContentDB(contentKey: [[UserContentKey]]((usercontentkey))) -> (data: buffer)
```

</td></tr>
<tr><td>9</td><td>

```swift
func GetContentURL(contentKey: [[UserContentKey]]((usercontentkey))) -> (downloadInfo: [[UserContentURL]]((usercontenturl)))
```

</td></tr>
<tr><td>10</td><td>

```swift
func GetSlotCount(typeID: uint32) -> (slotCount: [[UserSlotCount]]((userslotcount)))
```

</td></tr>
<tr><td>11</td><td>

```swift
func GetMetaData(contentKey: [[UserContentKey]]((usercontentkey))) -> (content: [[UserContent]]((usercontent)))
```

</td></tr>
<tr><td>12</td><td>

```swift
func Like(contentKey: [[UserContentKey]]((usercontentkey))) -> ()
```

</td></tr>
<tr><td>13</td><td>

```swift
func Unlike(contentKey: [[UserContentKey]]((usercontentkey))) -> ()
```

</td></tr>
<tr><td>14</td><td>

```swift
func IsLiked(contentKey: [[UserContentKey]]((usercontentkey))) -> (liked: bool)
```

</td></tr>
<tr><td>15</td><td>

```swift
func GetFavourites(contentTypes: qlist<uint32>) -> (favourites: qlist<[[UserContent]]((usercontent))>)
```

</td></tr>
<tr><td>16</td><td>

```swift
func MakeFavourite(contentKey: [[UserContentKey]]((usercontentkey))) -> ()
```

</td></tr>
<tr><td>17</td><td>

```swift
func RemoveFromFavourites(contentKey: [[UserContentKey]]((usercontentkey))) -> ()
```

</td></tr>
<tr><td>18</td><td>

```swift
func ReportInappropriate(contentKey: [[UserContentKey]]((usercontentkey)), reason: string) -> ()
```

</td></tr>
<tr><td>19</td><td>

```swift
func IncrementPlayCount(contentKey: [[UserContentKey]]((usercontentkey))) -> ()
```

</td></tr>
<tr><td>20</td><td>

```swift
func UpdateCustomStat(contentKey: [[UserContentKey]]((usercontentkey)), statID: uint16, incValue: int64) -> ()
```

</td></tr>
<tr><td>21</td><td>

```swift
func GetOwnContents(typeID: uint32) -> (results: qlist<[[UserContent]]((usercontent))>)
```

</td></tr>
<tr><td>22</td><td>

```swift
func GetMostPopularTags(contentKey: [[UserContentKey]]((usercontentkey))) -> (tags: qlist<[[WeightedTag]]((weightedtag))>, totalNumberOfTaggings: uint32)
```

</td></tr>
<tr><td>23</td><td>

```swift
func GetTags(contentKey: [[UserContentKey]]((usercontentkey))) -> (tagIds: qlist<uint32>)
```

</td></tr>
<tr><td>24</td><td>

```swift
func TagContent(contentKey: [[UserContentKey]]((usercontentkey)), newTagIds: qlist<uint32>) -> ()
```

</td></tr>
<tr><td>25</td><td>

```swift
func SearchContentsByPlayers(pids: qlist<uint32>, query: [[UserStorageQuery]]((userstoragequery))) -> (searchResults: qlist<[[UserContent]]((usercontent))>)
```

</td></tr>
<tr><td>26</td><td>

```swift
func SearchContentsByPlayersWithTotal(pids: qlist<uint32>, query: [[UserStorageQuery]]((userstoragequery))) -> (searchResults: qlist<[[UserContent]]((usercontent))>, totalResults: uint32)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
