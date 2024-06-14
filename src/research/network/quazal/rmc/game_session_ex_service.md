# GameSessionExService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 123 | [GameSessionExProtocol](#gamesessionexprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## GameSessionExProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func SearchSessions(gameSessionQuery: [[GameSessionQuery]]((gamesessionquery))) -> (searchResults: qlist<[[GameSessionSearchResultEx]]((gamesessionsearchresultex))>)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
