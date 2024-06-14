# UbiAccountManagementService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 29 | [UbiAccountManagementProtocol](#ubiaccountmanagementprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## UbiAccountManagementProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func CreateAccount() -> (failedReasons: qvector<[[ValidationFailureReason]]((validationfailurereason))>)
```

</td></tr>
<tr><td>2</td><td>

```swift
func UpdateAccount() -> (failedReasons: qvector<[[ValidationFailureReason]]((validationfailurereason))>)
```

</td></tr>
<tr><td>3</td><td>

```swift
func GetAccount() -> (ubiAccount: [[UbiAccount]]((ubiaccount)), exists: bool)
```

</td></tr>
<tr><td>4</td><td>

```swift
func LinkAccount(ubiAccountUsername: string, ubiAccountPassword: string) -> ()
```

</td></tr>
<tr><td>5</td><td>

```swift
func GetTOS(countryCode: string, languageCode: string, htmlVersion: bool) -> (tos: [[TOS]]((tos)))
```

</td></tr>
<tr><td>6</td><td>

```swift
func ValidateUsername(username: string) -> (usernameValidation: [[UsernameValidation]]((usernamevalidation)))
```

</td></tr>
<tr><td>7</td><td>

```swift
func ValidatePassword(password: string, username: string) -> (failedReasons: qvector<[[ValidationFailureReason]]((validationfailurereason))>)
```

</td></tr>
<tr><td>8</td><td>

```swift
func ValidateEmail(email: string) -> (failedReasons: qvector<[[ValidationFailureReason]]((validationfailurereason))>)
```

</td></tr>
<tr><td>9</td><td>

```swift
func GetCountryList(languageCode: string) -> (countries: qvector<[[Country]]((country))>)
```

</td></tr>
<tr><td>10</td><td>

```swift
func ForgetPassword(usernameOrEmail: string) -> ()
```

</td></tr>
<tr><td>11</td><td>

```swift
func LookupPrincipalIds(ubiAccountIds: std_list<string>) -> (pids: std_map<string, uint32>)
```

</td></tr>
<tr><td>12</td><td>

```swift
func LookupUbiAccountIDsByPids(pids: std_list<uint32>) -> (ubiaccountIDs: std_map<uint32, string>)
```

</td></tr>
<tr><td>13</td><td>

```swift
func LookupUbiAccountIDsByUsernames(Usernames: std_list<string>) -> (UbiAccountIDs: std_map<string, string>)
```

</td></tr>
<tr><td>14</td><td>

```swift
func LookupUsernamesByUbiAccountIDs(UbiAccountIds: std_list<string>) -> (Usernames: std_map<string, string>)
```

</td></tr>
<tr><td>15</td><td>

```swift
func LookupUbiAccountIDsByUsernameSubString(UsernameSubString: string) -> (UbiAccountIDs: std_map<string, string>)
```

</td></tr>
<tr><td>16</td><td>

```swift
func UserHasPlayed(UbiAccountIDs: std_list<string>) -> (UserPresence: std_map<string, bool>)
```

</td></tr>
<tr><td>17</td><td>

```swift
func IsUserPlaying(UbiAccountIDs: std_list<string>) -> (UserPresence: std_map<string, bool>)
```

</td></tr>
<tr><td>18</td><td>

```swift
func LookupUbiAccountIDsByUsernamesGlobal(Usernames: std_list<string>) -> (UbiAccountIDs: std_map<string, string>)
```

</td></tr>
<tr><td>19</td><td>

```swift
func LookupUbiAccountIDsByEmailsGlobal(Emails: std_list<string>) -> (UbiAccountIDs: std_map<string, string>)
```

</td></tr>
<tr><td>20</td><td>

```swift
func LookupUsernamesByUbiAccountIDsGlobal(UbiAccountIds: std_list<string>) -> (Usernames: std_map<string, string>)
```

</td></tr>
<tr><td>21</td><td>

```swift
func GetTOSEx(countryCode: string, languageCode: string, htmlVersion: bool) -> (tosex: [[TOSEx]]((tosex)))
```

</td></tr>
<tr><td>22</td><td>

```swift
func HasAcceptedLatestTOS() -> (hasAccepted: bool, failedReasons: qvector<[[ValidationFailureReason]]((validationfailurereason))>)
```

</td></tr>
<tr><td>23</td><td>

```swift
func AcceptLatestTOS() -> (failedReasons: qvector<[[ValidationFailureReason]]((validationfailurereason))>)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
