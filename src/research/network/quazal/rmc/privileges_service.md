# PrivilegesService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 35 | [PrivilegesProtocol](#privilegesprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## PrivilegesProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func GetPrivileges(localeCode: string) -> (privileges: std_map<uint32, [[Privilege]]((privilege))>)
```

</td></tr>
<tr><td>2</td><td>

```swift
func ActivateKey(uniqueKey: string, languageCode: string) -> (privilege: [[PrivilegeGroup]]((privilegegroup)))
```

</td></tr>
<tr><td>3</td><td>

```swift
func ActivateKeyWithExpectedPrivileges(uniqueKey: string, languageCode: string, expectedPrivilegeIDs: qlist<uint32>) -> (privilege: [[PrivilegeGroup]]((privilegegroup)))
```

</td></tr>
<tr><td>4</td><td>

```swift
func GetPrivilegeRemainDuration(privilegeID: uint32) -> (seconds: int32)
```

</td></tr>
<tr><td>5</td><td>

```swift
func GetExpiredPrivileges() -> (expiredPrivileges: qlist<[[PrivilegeEx]]((privilegeex))>)
```

</td></tr>
<tr><td>6</td><td>

```swift
func GetPrivilegesEx(localeCode: string) -> (privilegesEx: qlist<[[PrivilegeEx]]((privilegeex))>)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
