

# Column Codes

Management Console column codes for API

When using certain JSON-RPC API methods, you may be given or asked for column vectors or column codes and data source codes.

> 🚧 Please note, the old legacy shortnames will still work for the time being, but we would strongly recommend you change the API methods to use the below notations as soon as possible.

## Example

If you want to output the information from the creation date, computer name, used storage and active data sources columns for a list of devices. Use the Enumerating Account Statistics method.

Using the "Columns" parameter in the **old notation**, this would be written as:

```json
"Columns": ["CD", "MN", "US", "AP"],
```

Using the **new notation**, you are replacing "CD", "MN", "US" and "AP" with "I4", "I18", "I14" and "I78". This would mean the parameter in your JSON call now looks as below:

```json
"Columns": ["I4", "I18", "I14", "I78"],
```

> 📘 There is no limit to the number of column vectors that can be requested using this Column parameter.

The response showing this information will be displayed as:

```json
"Settings": [
    {
	"I4": "1536906195"
    },
    {
	"I18": "ComputerName1"
    },
    {
	"I14": "188702358870"
    },
    {
	"I78": "D1,D2"
    }
]
```

## Example Breakdown

In the above example, the Columns parameter is calling the following Settings to be displayed:

* I4: Creation Date\
  The response for this has come back as "1536906195", which is the date and time in Unix format. Converted, this is Sept 14th, 2018, 07:23:15 relative to my timezone
* I18: Computer Name\
  This is the unique computer name for the device in question
* I14: Used Storage\
  The response for this has come back as "188702358870", which is the total size of storage used for the device in Bytes. Converted, this is 188.7 GB.
* I78: Active Data Sources\
  The data sources active on the device as denoted by their respective ID numbers. The response here is D1 and D2, which mean that the Files and Folders and System State Data Sources are active on the device.

## Expressions for active data sources

See the list of Backup data sources here, with the legacy shortnames detailed for ease:

| Full Name                  | New ID | Legacy Shortname |
| -------------------------- | ------ | ---------------- |
| Files and Folders          | D1     | F                |
| System State               | D2     | S                |
| MsSql                      | D3     | Q                |
| VssExchange                | D4     | X                |
| Microsoft 365 SharePoint   | D5     | --               |
| NetworkShares              | D6     | N                |
| VssSystemState             | D7     | S                |
| VMware Virtual Machines    | D8     | W                |
| Total                      | D9     | T                |
| VssMsSql                   | D10    | Z                |
| VssSharePoint              | D11    | P                |
| Oracle                     | D12    | Y                |
| Hyper-V                    | D14    | H                |
| MySql                      | D15    | L                |
| Virtual Disaster Recovery  | D16    | V                |
| Bare Metal Restore         | D17    | B                |
| Microsoft 365 Exchange     | D19    | G                |
| Microsoft 365 OneDrive     | D20    | J                |
| Microsoft 365 Teams	D23	-- | D23    | --               |
| Removable Media            | --     | R                |

## Column Codes

See the list of Backup column codes below:

### Primary device properties

| Column Title      | New ID | Legacy Shortname | Type of data |
| ----------------- | ------ | ---------------- | ------------ |
| Device ID         | I0     | AU               | String       |
| Device name       | I1     | AN               | String       |
| Device name alias | I2     | AL               | String       |
| Password          | I3     | QW               | String       |
| Creation date     | I4     | CD               | Time         |
| Expiration date   | I5     | ED               | Time         |
| Customer          | I8     | AR               | String       |
| Product ID        | I9     | PD               | Int          |
| Product           | I10    | PN               | String       |
| Email             | I15    | EM               | String       |
| Retention units   | I39    | RU               | String       |
| Profile ID        | I54    | OI               | Int          |

### Installation details

<Table>
  <thead>
    <tr>
      <th>
        Column Title
      </th>

      <th>
        New ID
      </th>

      <th>
        Legacy Shortname
      </th>

      <th>
        Type of data
      </th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>
        OS version ?
      </td>

      <td>
        I16
      </td>

      <td>
        OS
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Client version
      </td>

      <td>
        I17
      </td>

      <td>
        VN
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Computer name
      </td>

      <td>
        I18
      </td>

      <td>
        MN
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Internal IPs
      </td>

      <td>
        I19
      </td>

      <td>
        IP
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        MAC address
      </td>

      <td>
        I21
      </td>

      <td>
        MA
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Time offset
      </td>

      <td>
        I24
      </td>

      <td>
        TZ
      </td>

      <td>
        Number
      </td>
    </tr>

    <tr>
      <td>
        OS type ?
      </td>

      <td>
        I32
      </td>

      <td>
        OT
      </td>

      <td>
        1 – workstation\
        2 – server\
        0 – undefined
      </td>
    </tr>

    <tr>
      <td>
        Computer manufacturer
      </td>

      <td>
        I44
      </td>

      <td>
        MF
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Computer model
      </td>

      <td>
        I45
      </td>

      <td>
        MO
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Installation ID
      </td>

      <td>
        I46
      </td>

      <td>
        II
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Installation Mode
      </td>

      <td>
        I47
      </td>

      <td>
        IM
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Unattended Installation account ID
      </td>

      <td>
        I74
      </td>

      <td>
        AI
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        First Installation Flag
      </td>

      <td>
        I75
      </td>

      <td>
        IF
      </td>

      <td>
        Int
      </td>
    </tr>
  </tbody>
</Table>

### Storage info

<Table>
  <thead>
    <tr>
      <th>
        Column Title
      </th>

      <th>
        New ID
      </th>

      <th>
        Legacy Shortname
      </th>

      <th>
        Type of data
      </th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>
        Storage location ?
      </td>

      <td>
        I11
      </td>

      <td>
        LN
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Used storage
      </td>

      <td>
        I14
      </td>

      <td>
        US
      </td>

      <td>
        Size
      </td>
    </tr>

    <tr>
      <td>
        Cabinet Storage Efficiency
      </td>

      <td>
        I26
      </td>

      <td>
        SE
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Total Cabinets Count
      </td>

      <td>
        I27
      </td>

      <td>
        CC
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Efficient Cabinet Count 0-25
      </td>

      <td>
        I28
      </td>

      <td>
        E0
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Efficient Cabinet Count 26-50
      </td>

      <td>
        I29
      </td>

      <td>
        E1
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Efficient Cabinet Count 50-75
      </td>

      <td>
        I30
      </td>

      <td>
        E2
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Used Virtual Storage
      </td>

      <td>
        I31
      </td>

      <td>
        UV
      </td>

      <td>
        Size
      </td>
    </tr>

    <tr>
      <td>
        Storage status
      </td>

      <td>
        I36
      </td>

      <td>
        YS
      </td>

      <td>
        -2 – Offline\
        -1 – Failed\
        0 – Undefined\
        50 – Running\
        100 – Synchronized
      </td>
    </tr>
  </tbody>
</Table>

### Feature usage

<Table>
  <thead>
    <tr>
      <th>
        Column Title
      </th>

      <th>
        New ID
      </th>

      <th>
        Legacy Shortname
      </th>

      <th>
        Type of data
      </th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>
        Active data sources
      </td>

      <td>
        I78
      </td>

      <td>
        AP
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Seeding mode
      </td>

      <td>
        I33
      </td>

      <td>
        IS
      </td>

      <td>
        0 – Undefined\
        1 – Normal\
        2 – Seeding\
        3 – PreSeeding\
        4 – PostSeeding
      </td>
    </tr>

    <tr>
      <td>
        LSV ?
      </td>

      <td>
        I35
      </td>

      <td>
        VE
      </td>

      <td>
        0 – Disabled\
        1 – Enabled
      </td>
    </tr>

    <tr>
      <td>
        LSV status
      </td>

      <td>
        I37
      </td>

      <td>
        YV
      </td>

      <td>
        -2 – Offline\
        -1 – Failed\
        0 – Undefined\
        50 – Running\
        100 – Synchronized
      </td>
    </tr>
  </tbody>
</Table>

### Data source statistics fields

<Table>
  <thead>
    <tr>
      <th>
        Column title
      </th>

      <th>
        NeW ID
      </th>

      <th>
        Legacy Shortname
      </th>

      <th>
        Type of data
      </th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>
        Last Session Status
      </td>

      <td>
        F00
      </td>

      <td>
        0
      </td>

      <td>
        1 – In process\
        2 – Failed\
        3 – Aborted\
        5 – Completed\
        6 – Interrupted\
        7 – NotStarted\
        8 – CompletedWithErrors\
        9 – InProgressWithFaults\
        10 – OverQuota\
        11 – NoSelection\
        12 – Restarted
      </td>
    </tr>

    <tr>
      <td>
        Last Session Selected Count
      </td>

      <td>
        F01
      </td>

      <td>
        1
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Last Session Processed Count
      </td>

      <td>
        F02
      </td>

      <td>
        2
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Last Session Selected Size
      </td>

      <td>
        F03
      </td>

      <td>
        3
      </td>

      <td>
        Size
      </td>
    </tr>

    <tr>
      <td>
        Last Session Processed Size
      </td>

      <td>
        F04
      </td>

      <td>
        4
      </td>

      <td>
        Size
      </td>
    </tr>

    <tr>
      <td>
        Last Session Sent Size
      </td>

      <td>
        F05
      </td>

      <td>
        5
      </td>

      <td>
        Size
      </td>
    </tr>

    <tr>
      <td>
        Last Session Errors Count
      </td>

      <td>
        F06
      </td>

      <td>
        7
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Protected size
      </td>

      <td>
        F07
      </td>

      <td>
        6
      </td>

      <td>
        Size
      </td>
    </tr>

    <tr>
      <td>
        Color bar – last 28 days
      </td>

      <td>
        F08
      </td>

      <td>
        B
      </td>

      <td>
        ColourBar
      </td>
    </tr>

    <tr>
      <td>
        Last successful session Time stamp
      </td>

      <td>
        F09
      </td>

      <td>
        L
      </td>

      <td>
        Time
      </td>
    </tr>

    <tr>
      <td>
        Pre Recent Session Selected Count
      </td>

      <td>
        F10
      </td>

      <td>
        8
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Pre Recent Session Selected Size
      </td>

      <td>
        F11
      </td>

      <td>
        9
      </td>

      <td>
        Size
      </td>
    </tr>

    <tr>
      <td>
        Session duration
      </td>

      <td>
        F12
      </td>

      <td>
        A
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Last Session License Items count ?
      </td>

      <td>
        F13
      </td>

      <td>
        I
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Retention
      </td>

      <td>
        F14
      </td>

      <td>
        R
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Last Session Time stamp
      </td>

      <td>
        F15
      </td>

      <td>
        G
      </td>

      <td>
        Time
      </td>
    </tr>

    <tr>
      <td>
        Last Successful Session Status
      </td>

      <td>
        F16
      </td>

      <td>
        Q
      </td>

      <td>
        Status
      </td>
    </tr>

    <tr>
      <td>
        Last Completed Session Status
      </td>

      <td>
        F17
      </td>

      <td>
        J
      </td>

      <td>
        Status
      </td>
    </tr>

    <tr>
      <td>
        Last Completed Session Time stamp
      </td>

      <td>
        F18
      </td>

      <td>
        O
      </td>

      <td>
        Time
      </td>
    </tr>

    <tr>
      <td>
        Last Session Verification Data
      </td>

      <td>
        F19
      </td>

      <td>
        K
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Last Session User Mailboxes Count
      </td>

      <td>
        F20
      </td>

      <td>
        M
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Last Session Shared Mailboxes Count ?
      </td>

      <td>
        F21
      </td>

      <td>
        @
      </td>

      <td>
        Int
      </td>
    </tr>
  </tbody>
</Table>

### Company Information

| Column Title            | New ID | Legacy Shortname | Type of data |
| ----------------------- | ------ | ---------------- | ------------ |
| Company Name            | I63    | NC               | String       |
| Address                 | I64    | AD               | String       |
| Zip Code                | I65    | ZP               | String       |
| Country                 | I66    | CY               | String       |
| City                    | I67    | CT               | String       |
| Phone Number            | I68    | PH               | String       |
| Fax Number              | I69    | FX               | String       |
| Contract Name           | I70    | CP               | String       |
| Group Name              | I71    | GN               | String       |
| Demo                    | I72    | DE               | Int          |
| Edu                     | I73    | EU               | Int          |
| Maximum Allowed Version | I76    | MV               | String       |

### Miscellaneous

<Table>
  <thead>
    <tr>
      <th>
        Column Title
      </th>

      <th>
        New ID
      </th>

      <th>
        Legacy Shortname
      </th>

      <th>
        Type of data
      </th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>
        Time stamp
      </td>

      <td>
        I6
      </td>

      <td>
        TS
      </td>

      <td>
        Unix time
      </td>
    </tr>

    <tr>
      <td>
        Device group name
      </td>

      <td>
        I12
      </td>

      <td>
        AG
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Own user name
      </td>

      <td>
        I13
      </td>

      <td>
        OU
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        External IPs
      </td>

      <td>
        I20
      </td>

      <td>
        EI
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Dashboard frequency
      </td>

      <td>
        I22
      </td>

      <td>
        DF
      </td>

      <td>
        Bitmask
      </td>
    </tr>

    <tr>
      <td>
        Dashboard language
      </td>

      <td>
        I23
      </td>

      <td>
        DL
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Anti Crypto enabled
      </td>

      <td>
        I34
      </td>

      <td>
        AC
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Archived size
      </td>

      <td>
        I38
      </td>

      <td>
        AS
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Activity description
      </td>

      <td>
        I40
      </td>

      <td>
        DS
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Number of Hyper-V virtual machines
      </td>

      <td>
        I41
      </td>

      <td>
        HN
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Number of ESX virtual machines
      </td>

      <td>
        I42
      </td>

      <td>
        EN
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Encryption status
      </td>

      <td>
        I43
      </td>

      <td>
        ES
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Restore email
      </td>

      <td>
        I48
      </td>

      <td>
        REM
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Restore dashboard frequency
      </td>

      <td>
        I49
      </td>

      <td>
        RDF
      </td>

      <td>
        Bitmask
      </td>
    </tr>

    <tr>
      <td>
        Restore dashboards language
      </td>

      <td>
        I50
      </td>

      <td>
        RDL
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Profile version
      </td>

      <td>
        I55
      </td>

      <td>
        OV
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Profile
      </td>

      <td>
        I56
      </td>

      <td>
        OP
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Stock Keeping Unit
      </td>

      <td>
        I57
      </td>

      <td>
        KU
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Stock Keeping Unit of the previous month
      </td>

      <td>
        I58
      </td>

      <td>
        PU
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Account type
      </td>

      <td>
        I59
      </td>

      <td>
        AT
      </td>

      <td>
        0 – Unknown\
        1 – Backup Manager\
        2 – M365
      </td>
    </tr>

    <tr>
      <td>
        Proxy Type
      </td>

      <td>
        I60
      </td>

      <td>
        PT
      </td>

      <td>
        Int
      </td>
    </tr>

    <tr>
      <td>
        Most Recent Restore Plug-in
      </td>

      <td>
        I62
      </td>

      <td>
        RP
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Customer reference
      </td>

      <td>
        I77
      </td>

      <td>
        PF
      </td>

      <td>
        String
      </td>
    </tr>

    <tr>
      <td>
        Recovery Testing
      </td>

      <td>
        I80
      </td>

      <td>
        \--
      </td>

      <td>
        Disabled\
        Enabled
      </td>
    </tr>

    <tr>
      <td>
        Physicality
      </td>

      <td>
        I81
      </td>

      <td>
        \--
      </td>

      <td>
        Undefined\
        Physical\
        Virtual
      </td>
    </tr>

    <tr>
      <td>
        Passphrase
      </td>

      <td>
        I82
      </td>

      <td>
        \--
      </td>

      <td>
        Yes\
        No
      </td>
    </tr>
  </tbody>
</Table>