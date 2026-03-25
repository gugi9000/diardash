
# Enumerating Devices

You can get the list of devices of your own company and your customers using the EnumerateAccounts method.

## Required parameters

<Table>
  <thead>
    <tr>
      <th>
        Parameter
      </th>

      <th>
        Description
      </th>

      <th>
        Supported values
      </th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>
        partnerId
      </td>

      <td>
        The ID of the customer whose devices you wish to list (retrieved through the GetPartnerInfo method)
      </td>

      <td>
        \<int> Integer
      </td>
    </tr>
  </tbody>
</Table>

## Sample Request

```json
{
    "id":"jsonrpc",
    "jsonrpc":"2.0",
    "visa": "{{visa}}",
    "method" : "EnumerateAccounts",
    "params" : {
	    "partnerId" : 123456
    }
}
```

## Sample response

```json
{
    "id": "jsonrpc",
    "jsonrpc": "2.0",
    "result": {
    "result": [
	{
	    "CreationTime": 1543482480,
	    "ExpirationTime": 2147483647,
	    "Id": 135792,
	    "LocationId": 1,
	    "Name": "Testpc1",
	    "NameAlias": null,
	    "PartnerId": 123456,
	    "Password": "111222c11011",
	    "ProductId": 56789,
	    "RemovalTime": 0,
	    "StorageLocationId": 1,
	    "Token": "9814bf0b-3a13-54jd-j972-0000XX0000X0Xx0",
	    "Type": "BackupManager"
	},
			
	{
	    "CreationTime": 1547372143,
	    "ExpirationTime": 2147483647,
	    "Id": 246801,
	    "LocationId": 1,
	    "Name": "Pctestingenviron1",
	    "NameAlias": null,
	    "PartnerId": 123456,
	    "Password": "11ab22c33d44",
	    "ProductId": 56789,
	    "RemovalTime": 0,
	    "StorageLocationId": 1,
	    "Token": "530mw397-85dq-47hh-01d5-0000XX0000X0Xx0",
	    "Type": "BackupManager"
	}
    ]
    },
    "visa": "{{visa}}"
}
```