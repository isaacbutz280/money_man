# Money Man

A budgeting app based on the Envelope budgeting method.

## Getting started

To begin, make sure Rust and Cargo are installed - https://www.rust-lang.org/tools/install

Make sure to ensure cargo is installed by running `cargo --version` in a Command Prompt.

### Windows

* From [Tags](https://github.com/isaacbutz280/money_man/tags), select the lastest version. NOTE - must be version v.0.2.0 or later.
* Download the zip file associated with the tag.
* Naviagte to the Downloads folder in file explorer and find 'money_man-X.X.X.zip', where 'X.X.X' is the version number.
* Extract the zip to any temporary location. C:\Users\<Username>\Downloads\ is recommended.
* In this temporary location should now be a 'money_man-X.X.X' folder, open it.
* Run the 'install.bat' script. This will build the application and create a desktop shortcut.
* Start budgeting!

### Linux
Not supported at the moment!

### MacOS
Not supported at the moment!

## Adding Envelopes

Begin by navigating to the 'Vope Mgr' tab. Adding the '+' button allows new envelopes to be created. There will already be a default envelope of 'Saftey'. Add as many as desired, giving each a unique name! Use the 'edit' button to delete envelopes. Don't delete the 'Saftey' envelope!

## Categorize!

Categorize your transactions by clicking 'start' under the 'Assign' tab. You will be prompted to select a '.csv' file containing all transactions to be categorized (see [Transactions CSV format](#transactions-csv-format) for details). The details will appear at the top of the page, and a cateogry to assign the transaction can be selected. Click the '->' button to categorize the transaction! You can see the envelope 'Actual' value be adjusted on the right side.

## Additional information

### Transactions CSV format

For Money Man to properly parse the '.csv' file provided, it must be of format

| Date       | Description                               | Amount |
| -----------|-------------------------------------------|--------|
| MM/DD/YYYY | Text that doesn't contain a `\|` character | float  |
| ...        | ...                                       | ...    |
| ...        | ...                                       | ...    |


The float value must:
* Be negative if money was spent (i.e buying ice cream)
* Be positive if money was earned (i.e paycheck hit!)
* NOT contain dollar signs
