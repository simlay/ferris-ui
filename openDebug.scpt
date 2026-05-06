#!/usr/bin/env osascript

on run argv
    if count of argv < 1 then
        logEvent("Usage: ./openDebug.scpt <PID>")
        error number -128
    end

    set pid_number to item 1 of argv

    tell application "Xcode" to activate

    try
        tell application "System Events"
            tell process "Xcode"
                click menu item "Attach to Process by PID or Name…" of menu "Debug" of menu bar 1
                delay 0.5
                keystroke pid_number
                delay 0.2
                keystroke return
                delay 0.1
                keystroke return
            end tell
        end tell
    on error errMsg
        logEvent ("Error: " & errMsg)
    end try
end run


on logEvent(themessage)
    -- All the recent events, results and error can be found at the give path
    -- It can easily be opened with the regular "Console" app
    set theLine to (do shell script "date  +'%Y-%m-%d %H:%M:%S'" as string) & " " & themessage
    do shell script "echo '" & theLine & "' >> ~/Library/Logs/AppleScript-events.log"
end log_event
