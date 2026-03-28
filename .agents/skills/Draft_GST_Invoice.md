---
name: Draft GST Invoice
description: Generates a compliant Indian GST invoice given transaction details.
---

When the user asks to draft a GST invoice, output the invoice strictly in the following Markdown format. Ask the user for any missing details (like GSTIN or Tax Rate) before generating.

# TAX INVOICE
**Invoice No:** [Auto-generate or ask]
**Date:** [Current Date]

**From:**
[User Name / Company]
[User Address]
GSTIN: [User GSTIN]

**To:**
[Client Name]
[Client Address]
GSTIN: [Client GSTIN, if any]

---

| Description of Service | SAC Code | Amount (₹) |
|------------------------|----------|------------|
| [Service details]      | [Code]   | [Amount]   |

**Total Before Tax:** ₹[Amount]

**GST Calculation:**
- If client is in same state as user: Add CGST (9%) and SGST (9%).
- If client is in a different state: Add IGST (18%).
- *If Zero-Rated (Export):* Note as "Export of Services under LUT. No tax payable."

**Grand Total:** ₹[Final Amount]

---

**Bank Details:**
Account Name: [Name]
Account No: [Number]
IFSC: [Code]
