Set WshShell = CreateObject("WScript.Shell")
' Run the batch file hidden (0)
WshShell.Run chr(34) & "WorkmAIte_PRO_Launcher.bat" & Chr(34), 0
Set WshShell = Nothing
