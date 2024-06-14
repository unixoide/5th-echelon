# GameSessionService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 42 | [GameSessionProtocol](#gamesessionprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## GameSessionProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func CreateSession(gameSession: [[GameSession]]((gamesession))) -> (gameSessionKey: [[GameSessionKey]]((gamesessionkey)))
```

</td></tr>
<tr><td>2</td><td>

```swift
func UpdateSession(gameSessionUpdate: [[GameSessionUpdate]]((gamesessionupdate))) -> ()
```

</td></tr>
<tr><td>3</td><td>

```swift
func DeleteSession(gameSessionKey: [[GameSessionKey]]((gamesessionkey))) -> ()
```

</td></tr>
<tr><td>4</td><td>

```swift
func MigrateSession(gameSessionKey: [[GameSessionKey]]((gamesessionkey))) -> (gameSessionKeyMigrated: [[GameSessionKey]]((gamesessionkey)))
```

</td></tr>
<tr><td>5</td><td>

```swift
func LeaveSession(gameSessionKey: [[GameSessionKey]]((gamesessionkey))) -> ()
```

</td></tr>
<tr><td>6</td><td>

```swift
func GetSession(gameSessionKey: [[GameSessionKey]]((gamesessionkey))) -> (searchResult: [[GameSessionSearchResult]]((gamesessionsearchresult)))
```

</td></tr>
<tr><td>7</td><td>

```swift
func SearchSessions(gameSessionQuery: [[GameSessionQuery]]((gamesessionquery))) -> (searchResults: qlist<[[GameSessionSearchResult]]((gamesessionsearchresult))>)
```

</td></tr>
<tr><td>8</td><td>

```swift
func AddParticipants(gameSessionKey: [[GameSessionKey]]((gamesessionkey)), publicParticipantIDs: qlist<uint32>, privateParticipantIDs: qlist<uint32>) -> ()
```

</td></tr>
<tr><td>9</td><td>

```swift
func RemoveParticipants(gameSessionKey: [[GameSessionKey]]((gamesessionkey)), participantIDs: qlist<uint32>) -> ()
```

</td></tr>
<tr><td>10</td><td>

```swift
func GetParticipantCount(gameSessionKey: [[GameSessionKey]]((gamesessionkey))) -> (count: uint32)
```

</td></tr>
<tr><td>11</td><td>

```swift
func GetParticipants(gameSessionKey: [[GameSessionKey]]((gamesessionkey)), resultRange: [[ResultRange]]((resultrange))) -> (participants: qlist<[[GameSessionParticipant]]((gamesessionparticipant))>)
```

</td></tr>
<tr><td>12</td><td>

```swift
func SendInvitation(invitation: [[GameSessionInvitation]]((gamesessioninvitation))) -> ()
```

</td></tr>
<tr><td>13</td><td>

```swift
func GetInvitationReceivedCount(gameSessionTypeID: uint32) -> (count: uint32)
```

</td></tr>
<tr><td>14</td><td>

```swift
func GetInvitationsReceived(gameSessionTypeID: uint32, resultRange: [[ResultRange]]((resultrange))) -> (invitations: qlist<[[GameSessionInvitationReceived]]((gamesessioninvitationreceived))>)
```

</td></tr>
<tr><td>15</td><td>

```swift
func GetInvitationSentCount(gameSessionTypeID: uint32) -> (count: uint32)
```

</td></tr>
<tr><td>16</td><td>

```swift
func GetInvitationsSent(gameSessionTypeID: uint32, resultRange: [[ResultRange]]((resultrange))) -> (invitations: qlist<[[GameSessionInvitationSent]]((gamesessioninvitationsent))>)
```

</td></tr>
<tr><td>17</td><td>

```swift
func AcceptInvitation(gameSessionInvitation: [[GameSessionInvitationReceived]]((gamesessioninvitationreceived))) -> ()
```

</td></tr>
<tr><td>18</td><td>

```swift
func DeclineInvitation(gameSessionInvitation: [[GameSessionInvitationReceived]]((gamesessioninvitationreceived))) -> ()
```

</td></tr>
<tr><td>19</td><td>

```swift
func CancelInvitation(gameSessionInvitation: [[GameSessionInvitationSent]]((gamesessioninvitationsent))) -> ()
```

</td></tr>
<tr><td>20</td><td>

```swift
func SendTextMessage(gameSessionMessage: [[GameSessionMessage]]((gamesessionmessage))) -> ()
```

</td></tr>
<tr><td>21</td><td>

```swift
func RegisterURLs(stationURLs: qlist<stationurl>) -> ()
```

</td></tr>
<tr><td>22</td><td>

```swift
func JoinSession(gameSessionKey: [[GameSessionKey]]((gamesessionkey))) -> ()
```

</td></tr>
<tr><td>23</td><td>

```swift
func AbandonSession(gameSessionKey: [[GameSessionKey]]((gamesessionkey))) -> ()
```

</td></tr>
<tr><td>24</td><td>

```swift
func SearchSessionsWithParticipants(gameSessionTypeID: uint32, participantIDs: qlist<uint32>) -> (searchResults: qlist<[[GameSessionSearchWithParticipantsResult]]((gamesessionsearchwithparticipantsresult))>)
```

</td></tr>
<tr><td>25</td><td>

```swift
func GetSessions(gameSessionKeys: qlist<[[GameSessionKey]]((gamesessionkey))>) -> (searchResults: qlist<[[GameSessionSearchResult]]((gamesessionsearchresult))>)
```

</td></tr>
<tr><td>26</td><td>

```swift
func GetParticipantsURLs(gameSessionKey: [[GameSessionKey]]((gamesessionkey)), participantIDs: qlist<uint32>) -> (participants: qlist<[[GameSessionParticipant]]((gamesessionparticipant))>)
```

</td></tr>
<tr><td>27</td><td>

```swift
func MigrateSessionHost(gameSessionKey: [[GameSessionKey]]((gamesessionkey))) -> ()
```

</td></tr>
<tr><td>28</td><td>

```swift
func SplitSession(gameSessionKey: [[GameSessionKey]]((gamesessionkey))) -> (gameSessionKeyMigrated: [[GameSessionKey]]((gamesessionkey)))
```

</td></tr>
<tr><td>29</td><td>

```swift
func SearchSocialSessions(gameSessionSocialQuery: [[GameSessionSocialQuery]]((gamesessionsocialquery))) -> (searchResults: qlist<[[GameSessionSearchWithParticipantsResult]]((gamesessionsearchwithparticipantsresult))>)
```

</td></tr>
<tr><td>30</td><td>

```swift
func ReportUnsuccessfulJoinSessions(unsuccessfulJoinSessions: qlist<[[GameSessionUnsuccessfulJoinSession]]((gamesessionunsuccessfuljoinsession))>) -> ()
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
