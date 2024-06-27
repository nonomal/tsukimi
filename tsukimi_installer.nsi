; Define installer name
Name "Tsukimi Installer"

; Define output file
OutFile "TsukimiSetup.exe"

; Default installation directory
InstallDir "$PROGRAMFILES\Tsukimi"

; Store previous installation location
Var PreviousInstallDir

; Request administrator privileges
RequestExecutionLevel admin

; Pages
Page directory
Page components
Page instfiles

; Uninstaller pages
UninstPage uninstConfirm
UninstPage instfiles

; Main installation section
Section "Tsukimi Main Program" SecMain
    SectionIn RO
    SetOutPath "$INSTDIR"
    
    ; Copy entire tsukimi folder
    File /r "tsukimi-windows-gnu-amd64\*.*"
    
    ; Create start menu shortcut
    CreateDirectory "$SMPROGRAMS\Tsukimi"
    CreateShortCut "$SMPROGRAMS\Tsukimi\Tsukimi.lnk" "$INSTDIR\bin\tsukimi.exe"
    
    ; Create uninstaller
    WriteUninstaller "$INSTDIR\Uninstall.exe"
    
    ; Write installation info to registry
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Tsukimi" "DisplayName" "Tsukimi"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Tsukimi" "UninstallString" '"$INSTDIR\Uninstall.exe"'
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Tsukimi" "InstallLocation" "$INSTDIR"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Tsukimi" "Publisher" "tsukinaha"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Tsukimi" "DisplayVersion" "0.6.0"
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Tsukimi" "NoModify" 1
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Tsukimi" "NoRepair" 1
SectionEnd

; Desktop shortcut section
Section "Create Desktop Shortcut" SecDesktopShortcut
    CreateShortCut "$DESKTOP\Tsukimi.lnk" "$INSTDIR\bin\tsukimi.exe"
SectionEnd

; Uninstall section
Section "Uninstall"
    ; Remove installed files
    RMDir /r "$INSTDIR"
    
    ; Remove start menu shortcut
    Delete "$SMPROGRAMS\Tsukimi\Tsukimi.lnk"
    RMDir "$SMPROGRAMS\Tsukimi"
    
    ; Remove desktop shortcut
    Delete "$DESKTOP\Tsukimi.lnk"
    
    ; Remove uninstaller
    Delete "$INSTDIR\Uninstall.exe"
    
    ; Remove registry keys
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Tsukimi"
SectionEnd

; Function to detect previous installation
Function .onInit
    ; Try to read previous installation directory
    ReadRegStr $PreviousInstallDir HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Tsukimi" "InstallLocation"
    StrCmp $PreviousInstallDir "" done
    
    ; If a previous installation is found, set the installation directory to the previous one
    StrCpy $INSTDIR $PreviousInstallDir
    
    done:
FunctionEnd