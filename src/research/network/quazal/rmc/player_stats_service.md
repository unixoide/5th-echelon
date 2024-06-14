# PlayerStatsService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 55 | [PlayerStatsProtocol](#playerstatsprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## PlayerStatsProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func WriteStats(playerStatUpdates: qlist<[[PlayerStatUpdate]]((playerstatupdate))>) -> ()
```

</td></tr>
<tr><td>2</td><td>

```swift
func ReadStatsByPlayers(playerPIDs: qlist<uint32>, queries: qlist<[[StatboardQuery]]((statboardquery))>) -> (results: qlist<[[StatboardResult]]((statboardresult))>)
```

</td></tr>
<tr><td>3</td><td>

```swift
func ReadLeaderboardsNearPlayer(playerPID: uint32, count: uint32, queries: qlist<[[LeaderboardQuery]]((leaderboardquery))>) -> (results: qlist<[[LeaderboardResult]]((leaderboardresult))>)
```

</td></tr>
<tr><td>4</td><td>

```swift
func ReadLeaderboardsByRank(startingRank: uint32, count: uint32, queries: qlist<[[LeaderboardQuery]]((leaderboardquery))>) -> (results: qlist<[[LeaderboardResult]]((leaderboardresult))>)
```

</td></tr>
<tr><td>5</td><td>

```swift
func ReadLeaderboardsByPlayers(playerPIDs: qlist<uint32>, queries: qlist<[[LeaderboardQuery]]((leaderboardquery))>) -> (results: qlist<[[LeaderboardResult]]((leaderboardresult))>)
```

</td></tr>
<tr><td>6</td><td>

```swift
func ReadStatboardHistory(queries: qlist<[[StatboardHistoryQuery]]((statboardhistoryquery))>) -> (results: qlist<[[StatboardResult]]((statboardresult))>)
```

</td></tr>
<tr><td>7</td><td>

```swift
func ReadLeaderboardHistory(queries: qlist<[[LeaderboardHistoryQuery]]((leaderboardhistoryquery))>) -> (results: qlist<[[LeaderboardResult]]((leaderboardresult))>)
```

</td></tr>
<tr><td>8</td><td>

```swift
func ReadStatboardHistoryAggregated(queries: qlist<[[StatboardHistoryAggregatedQuery]]((statboardhistoryaggregatedquery))>) -> (results: qlist<[[StatboardResult]]((statboardresult))>)
```

</td></tr>
<tr><td>9</td><td>

```swift
func GetStatboardNextPurgeDate(boardID: uint32, resetFrequency: uint32) -> (purgeDate: datetime)
```

</td></tr>
<tr><td>10</td><td>

```swift
func ReadLeaderboardsNearPlayer2(playerPID: uint32, count: uint32, queries: qlist<[[LeaderboardQuery2]]((leaderboardquery2))>) -> (results: qlist<[[LeaderboardResult]]((leaderboardresult))>)
```

</td></tr>
<tr><td>11</td><td>

```swift
func ReadLeaderboardsByRank2(startingRank: uint32, count: uint32, queries: qlist<[[LeaderboardQuery2]]((leaderboardquery2))>) -> (results: qlist<[[LeaderboardResult]]((leaderboardresult))>)
```

</td></tr>
<tr><td>12</td><td>

```swift
func ReadLeaderboardsByPlayers2(queries: qlist<[[LeaderboardQuery2]]((leaderboardquery2))>) -> (results: qlist<[[LeaderboardResult]]((leaderboardresult))>)
```

</td></tr>
<tr><td>13</td><td>

```swift
func ReadPopulationStats(queries: qlist<[[PopulationStatQuery]]((populationstatquery))>) -> (results: qlist<[[PopulationStatResult]]((populationstatresult))>)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
