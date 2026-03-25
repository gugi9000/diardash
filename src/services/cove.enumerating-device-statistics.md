

# Enumerating Device Statistics

You can get the statistics of devices of your own company and your customers using the EnumerateAccountStatistics method. A common use of this method is to output a list of storage space used on the cloud per device. You can get this information by using the Columns parameter and using the column I14.

> 🚧 Please be aware that there are no methods which can perform complex calculations, however the Totals parameter can do basic calculations using column codes. If you need to do complex calculations, you will have to take the given sizes (in Bytes) and do these manually.

## Required parameters

| Parameter | Description                                            | Supported values                                                                                                                                                  |
| --------- | ------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| query     | A group of parameters related to the device statistics | AccountStatisticsQuery (has child parameters of its own, see the [AccountStatisticsQuery child parameters ](#accountstatisticsquery-child-parameters)table below) |

### AccountStatisticsQuery child parameters

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
        PartnerId
      </td>

      <td>
        The ID of the customer the device is created for (retrieved through the GetPartnerInfo method)
      </td>

      <td>
        \<int> Integer
      </td>
    </tr>

    <tr>
      <td>
        Filter
      </td>

      <td>
        Apply a search parameter using RegularExpression (RegEx)
      </td>

      <td>
        \<std::string>
      </td>
    </tr>

    <tr>
      <td>
        ExcludedPartners
      </td>

      <td>
        A list of partner ID's to exclude from the search
      </td>

      <td>
        IdSet
      </td>
    </tr>

    <tr>
      <td>
        SelectionMode
      </td>

      <td>
        An array of selection modes
      </td>

      <td>
        AccountStatistics\
        SelectionMode::Enum

        Undefined\
        Merged\
        PerInstallation\
        Count
      </td>
    </tr>

    <tr>
      <td>
        Labels
      </td>

      <td>
        Any labels to display
      </td>

      <td>
        \<int>
      </td>
    </tr>

    <tr>
      <td>
        StartRecordNumber
      </td>

      <td>
        Which device number to start the output from
      </td>

      <td>
        \<int>
      </td>
    </tr>

    <tr>
      <td>
        RecordsCount
      </td>

      <td>
        How many devices to display
      </td>

      <td>
        \<int>
      </td>
    </tr>

    <tr>
      <td>
        OrderBy
      </td>

      <td>
        How to order the displayed list of results
      </td>

      <td>
        \<std::string>
      </td>
    </tr>

    <tr>
      <td>
        Columns
      </td>

      <td>
        Which column vectors you wish to display in the response
      </td>

      <td>
        ColumnVector\
        (see API Column Codes for all options)
      </td>
    </tr>

    <tr>
      <td>
        Totals
      </td>

      <td>
        An array of totals represented as strings, will return totalStatistics.

        For Example: merateAccountStatistics meth,

        This will return totalStatistics with the sum of used storage for the selected partner and filter and also a count of Cloud2Cloud devices
      </td>

      <td>
        TotalVector

        * SUM
        * COUNT
        * MAX
        * MIN
      </td>
    </tr>
  </tbody>
</Table>

## Sample request

```json
{
    "jsonrpc":"2.0",
    "id":"jsonrpc",
    "visa": "{{visa}}",
    "method" : "EnumerateAccountStatistics",
    "params" : {
	"query" : {
		"PartnerId" : 123456,
		"Filter": "ANY =~ 'Device*'",
		"SelectionMode": "Merged",
		"StartRecordNumber": 0,
		"RecordsCount": 3,
		"Columns": ["I1", "I14", "I18", "Do9F00", "D01F07"]
	}
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
		"AccountId": 654321,
		"Flags": [
		    "AutoDeployed"
		],
		"PartnerId": 123456,
		"Settings": [
		    {
			"I1": "computerName"
		    },
		    {
			"I14": "0"
		    },
		    {
			"I78": "D01D02"
		    }
		]
	    },
	    {
		"AccountId": 765432,
		"Flags": null,
		"PartnerId": 456789,
		"Settings": [
		    {
			"D01F07": "28349567768726"
		    },
		    {
			"D09F00": "1"
		    },
		    {
			"I1": "computerName2"
		    },
		    {
			"I14": "586755749630"
		    },
		    {
			"I78": "D01D02"
		    }
		]
	    },
	    {
		"AccountId": 876543,
		"Flags": null,
		"PartnerId": 456789,
		"Settings": [
		    {
			"D09F00": "5"
		    },
		    {
			"I1": "computerName3"
		    },
		    {
			"I14": "23630480"
		    },
		    {
			"I78": "D19D20D05"
		    }
		]
	    }
	],
	"totalStatistics": null
    },
    "visa": "{{visa}}"
}
```