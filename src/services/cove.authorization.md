# Authorization

Authorization in JSON-RPC API

Any request to the Management Service must be authorized. You can get access credentials from your service provider.

In order for authorization to be permitted, your user account in Management Console must have **API Authentication** enabled. This can be done either at the time of adding the user account, or after, by editing the user in Management Console and enabling **API Authentication**.

<Image align="center" width="60% " src="https://files.readme.io/22452230d7f770bfae1e5811df3daed0c0e6badef6e9376761bb18d324228949-image.png" />

> 📘 More information on user types and permissions can be found here.

In a response to your authorization request (Login), you will get a visa. This is a required parameter for all further requests. The visa stays valid for 15 minutes.

> 📘 Each response contains a new visa. You can use visas from previous calls to keep the visa chain uninterrupted. If the interval between service calls exceeds 15 minutes, you will need to repeat the Login request and start a new visa chain.

## Required parameters

| Parameter | Description                                       | Type/Supported values |
| --------- | ------------------------------------------------- | --------------------- |
| partner   | The name of the customer you want to log in under | std::string String    |
| username  | Your email address for access to the service      | std::string String    |
| password  | Your password for access to the service           | std::string String    |

### Sample request

```json
{
    "jsonrpc":"2.0",
    "method":"Login",
    "params":{
	    "partner":"Smart Telecom Inc.",
	    "username":"admin@smart-telecom.net",
	    "password":"sec1234!6"
    },
    "id":"1"
}
```

### Sample response

```json
{
    "id": "1",
    "jsonrpc": "2.0",
    "result": {
	"result": {
		"EmailAddress": "admin@smart-telecom.net",
		"FirstLoginTime": 1464945879,
		"FirstName": "Christine",
		"Flags": [
		    "AllowApiAuthentication"
		],
		"FullName": "Smith",
		"Id": 50193,
		"LastLoginTime": 1512383091,
		"Name": "admin@smart-telecom.net",
		"PartnerId": 33491,
		"Password": null,
		"PhoneNumber": "",
		"RoleId": 1,
		"Title": "Reseller",
		"TwoFactorAuthenticationStatus": "Enabled"
	}
    },
    "visa": "{{visa}}"
}
```