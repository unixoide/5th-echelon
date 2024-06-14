# UplayWinService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 49 | [UplayWinProtocol](#uplaywinprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## UplayWinProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func GetActions(startRowIndex: int32, maximumRows: int32, sortExpression: string, cultureName: string, platformCode: string, gameCode: string) -> (actionList: qlist<[[UplayAction]]((uplayaction))>)
```

</td></tr>
<tr><td>2</td><td>

```swift
func GetActionsCompleted(startRowIndex: int32, maximumRows: int32, sortExpression: string, cultureName: string, platformCode: string, gameCode: string) -> (actionList: qlist<[[UplayAction]]((uplayaction))>)
```

</td></tr>
<tr><td>3</td><td>

```swift
func GetActionsCount(platformCode: string, gameCode: string) -> (actionsCount: int32)
```

</td></tr>
<tr><td>4</td><td>

```swift
func GetActionsCompletedCount(platformCode: string, gameCode: string) -> (actionsCount: int32)
```

</td></tr>
<tr><td>5</td><td>

```swift
func GetRewards(startRowIndex: int32, maximumRows: int32, sortExpression: string, cultureName: string, platformCode: string, gameCode: string) -> (rewardList: qlist<[[UplayReward]]((uplayreward))>)
```

</td></tr>
<tr><td>6</td><td>

```swift
func GetRewardsPurchased(startRowIndex: int32, maximumRows: int32, sortExpression: string, cultureName: string, platformCode: string, gameCode: string) -> (rewardList: qlist<[[UplayReward]]((uplayreward))>)
```

</td></tr>
<tr><td>7</td><td>

```swift
func UplayWelcome(cultureName: string, platformCode: string, gameCode: string) -> (actionList: qlist<[[UplayAction]]((uplayaction))>)
```

</td></tr>
<tr><td>8</td><td>

```swift
func SetActionCompleted(actionCode: string, cultureName: string, platformCode: string, gameCode: string) -> (unlockedAction: [[UplayAction]]((uplayaction)))
```

</td></tr>
<tr><td>9</td><td>

```swift
func SetActionsCompleted(actionCodeList: qlist<string>, cultureName: string, platformCode: string, gameCode: string) -> (actionList: qlist<[[UplayAction]]((uplayaction))>)
```

</td></tr>
<tr><td>10</td><td>

```swift
func GetUserToken() -> (token: string)
```

</td></tr>
<tr><td>11</td><td>

```swift
func GetVirtualCurrencyUserBalance(platformCode: string) -> (virtualCurrencyUserBalance: int32)
```

</td></tr>
<tr><td>12</td><td>

```swift
func GetSectionsByKey(cultureName: string, sectionKey: string, platformCode: string, gameCode: string) -> (sectionList: qlist<[[UplaySection]]((uplaysection))>)
```

</td></tr>
<tr><td>13</td><td>

```swift
func BuyReward(rewardCode: string, platformCode: string) -> (virtualCurrencyUserBalance: int32)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
