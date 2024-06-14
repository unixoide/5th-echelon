# NewsService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| ? | [NewsProtocol](#newsprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## NewsProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func GetChannels(resultRange: [[ResultRange]]((resultrange))) -> (channels: qlist<[[NewsChannel]]((newschannel))>)
```

</td></tr>
<tr><td>2</td><td>

```swift
func GetChannelsByTypes(newsChannelTypes: qlist<string>, resultRange: [[ResultRange]]((resultrange))) -> (channels: qlist<[[NewsChannel]]((newschannel))>)
```

</td></tr>
<tr><td>3</td><td>

```swift
func GetSubscribableChannels(resultRange: [[ResultRange]]((resultrange))) -> (channels: qlist<[[NewsChannel]]((newschannel))>)
```

</td></tr>
<tr><td>4</td><td>

```swift
func GetChannelsByIDs(newsChannelIDs: qlist<uint32>) -> (channels: qlist<[[NewsChannel]]((newschannel))>)
```

</td></tr>
<tr><td>5</td><td>

```swift
func GetSubscribedChannels(resultRange: [[ResultRange]]((resultrange))) -> (channels: qlist<[[NewsChannel]]((newschannel))>)
```

</td></tr>
<tr><td>6</td><td>

```swift
func SubscribeChannel(newsChannelID: uint32) -> ()
```

</td></tr>
<tr><td>7</td><td>

```swift
func UnsubscribeChannel(newsChannelID: uint32) -> ()
```

</td></tr>
<tr><td>8</td><td>

```swift
func GetNewsHeaders(recipient: [[NewsRecipient]]((newsrecipient)), range: [[ResultRange]]((resultrange))) -> (newsHeaders: qlist<[[NewsHeader]]((newsheader))>)
```

</td></tr>
<tr><td>9</td><td>

```swift
func GetNewsMessages(newsMessageIDs: qlist<uint32>) -> (newsMessages: qlist<[[NewsMessage]]((newsmessage))>)
```

</td></tr>
<tr><td>10</td><td>

```swift
func GetNumberOfNews(recipient: [[NewsRecipient]]((newsrecipient))) -> (numberOfNews: uint32)
```

</td></tr>
<tr><td>11</td><td>

```swift
func GetChannelByType(newsChannelType: string) -> (channel: [[NewsChannel]]((newschannel)))
```

</td></tr>
<tr><td>12</td><td>

```swift
func GetNewsHeadersByType(newsChannelType: string, range: [[ResultRange]]((resultrange))) -> (newsHeaders: qlist<[[NewsHeader]]((newsheader))>)
```

</td></tr>
<tr><td>13</td><td>

```swift
func GetNewsMessagesByType(newsChannelType: string, range: [[ResultRange]]((resultrange))) -> (newsMessages: qlist<[[NewsMessage]]((newsmessage))>)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
