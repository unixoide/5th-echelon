# AccountManagementService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 25 | [AccountManagementProtocol](#accountmanagementprotocol) |
<!-- INSERT protocol_idx END -->


<!-- INSERT protocols START -->
## AccountManagementProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func CreateAccount(strPrincipalName: string, strKey: string, uiGroups: uint32, strEmail: string) -> (retval: qresult)
```

</td></tr>
<tr><td>2</td><td>

```swift
func DeleteAccount(idPrincipal: uint32) -> ()
```

</td></tr>
<tr><td>3</td><td>

```swift
func DisableAccount(idPrincipal: uint32, dtUntil: datetime, strMessage: string) -> (retval: qresult)
```

</td></tr>
<tr><td>4</td><td>

```swift
func ChangePassword(strNewKey: string) -> (retval: bool)
```

</td></tr>
<tr><td>5</td><td>

```swift
func TestCapability(uiCapability: uint32) -> (retval: bool)
```

</td></tr>
<tr><td>6</td><td>

```swift
func GetName(idPrincipal: uint32) -> (strName: string)
```

</td></tr>
<tr><td>7</td><td>

```swift
func GetAccountData() -> (retval: qresult, oAccountData: [[AccountData]]((accountdata)))
```

</td></tr>
<tr><td>8</td><td>

```swift
func GetPrivateData() -> (retval: bool, oData: any<[[Data]]((data)), string>)
```

</td></tr>
<tr><td>9</td><td>

```swift
func GetPublicData(idPrincipal: uint32) -> (retval: bool, oData: any<[[Data]]((data)), string>)
```

</td></tr>
<tr><td>10</td><td>

```swift
func GetMultiplePublicData(lstPrincipals: std_list<uint32>) -> (retval: bool, oData: std_list<any<[[Data]]((data)), string>>)
```

</td></tr>
<tr><td>11</td><td>

```swift
func UpdateAccountName(strName: string) -> (retval: qresult)
```

</td></tr>
<tr><td>12</td><td>

```swift
func UpdateAccountEmail(strName: string) -> (retval: qresult)
```

</td></tr>
<tr><td>13</td><td>

```swift
func UpdateCustomData(oPublicData: any<[[Data]]((data)), string>, oPrivateData: any<[[Data]]((data)), string>) -> (retval: qresult)
```

</td></tr>
<tr><td>14</td><td>

```swift
func FindByNameRegex(uiGroups: uint32, strRegex: string, resultRange: [[ResultRange]]((resultrange))) -> (plstAccounts: std_list<[[BasicAccountInfo]]((basicaccountinfo))>)
```

</td></tr>
<tr><td>15</td><td>

```swift
func UpdateAccountExpiryDate(idPrincipal: uint32, dtExpiry: datetime, strExpiredMessage: string) -> ()
```

</td></tr>
<tr><td>16</td><td>

```swift
func UpdateAccountEffectiveDate(idPrincipal: uint32, dtEffectiveFrom: datetime, strNotEffectiveMessage: string) -> ()
```

</td></tr>
<tr><td>17</td><td>

```swift
func UpdateStatus(strStatus: string) -> ()
```

</td></tr>
<tr><td>18</td><td>

```swift
func GetStatus(idPrincipal: uint32) -> (strStatus: string)
```

</td></tr>
<tr><td>19</td><td>

```swift
func GetLastConnectionStats(idPrincipal: uint32) -> (dtLastSessionLogin: datetime, dtLastSessionLogout: datetime, dtCurrentSessionLogin: datetime)
```

</td></tr>
<tr><td>20</td><td>

```swift
func ResetPassword() -> (retval: bool)
```

</td></tr>
<tr><td>21</td><td>

```swift
func CreateAccountWithCustomData(strPrincipalName: string, strKey: string, uiGroups: uint32, strEmail: string, oPublicData: any<[[Data]]((data)), string>, oPrivateData: any<[[Data]]((data)), string>) -> ()
```

</td></tr>
<tr><td>22</td><td>

```swift
func RetrieveAccount() -> (oAccountData: [[AccountData]]((accountdata)), oPublicData: any<[[Data]]((data)), string>, oPrivateData: any<[[Data]]((data)), string>)
```

</td></tr>
<tr><td>23</td><td>

```swift
func UpdateAccount(strKey: string, strEmail: string, oPublicData: any<[[Data]]((data)), string>, oPrivateData: any<[[Data]]((data)), string>) -> ()
```

</td></tr>
<tr><td>24</td><td>

```swift
func ChangePasswordByGuest(strPrincipalName: string, strEmail: string, strKey: string) -> ()
```

</td></tr>
<tr><td>25</td><td>

```swift
func FindByNameLike(uiGroups: uint32, strLike: string, resultRange: [[ResultRange]]((resultrange))) -> (plstAccounts: std_list<[[BasicAccountInfo]]((basicaccountinfo))>)
```

</td></tr>
<tr><td>26</td><td>

```swift
func CustomCreateAccount(strPrincipalName: string, strKey: string, uiGroups: uint32, strEmail: string, oAuthData: any<[[Data]]((data)), string>) -> (pid: uint32)
```

</td></tr>
<tr><td>27</td><td>

```swift
func LookupOrCreateAccount(strPrincipalName: string, strKey: string, uiGroups: uint32, strEmail: string, oAuthData: any<[[Data]]((data)), string>) -> (pid: uint32)
```

</td></tr>
<tr><td>28</td><td>

```swift
func CreateAccountEx(principalType: int8, strPrincipalName: string, strKey: string, uiGroups: uint32, strEmail: string, context: uint64) -> (retval: qresult)
```

</td></tr>
<tr><td>29</td><td>

```swift
func DisconnectPrincipal(idPrincipal: uint32) -> (retval: bool)
```

</td></tr>
<tr><td>30</td><td>

```swift
func DisconnectAllPrincipals() -> (retval: bool)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
