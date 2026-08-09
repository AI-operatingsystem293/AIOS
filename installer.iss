#define MyAppName "AIOS"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "AI-operatingsystem293"
#define MyAppExeName "aios.exe"

[Setup]
AppId={{AIOS-0.1.0}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={autopf}\AIOS
DefaultGroupName=AIOS
OutputDir=dist
OutputBaseFilename=AIOS-Setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern

[Files]
Source: "target\release\aios.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\AIOS"; Filename: "{app}\aios.exe"
Name: "{autodesktop}\AIOS"; Filename: "{app}\aios.exe"

[Run]
Filename: "{app}\aios.exe"
Description: "Start AIOS"
Flags: nowait postinstall skipifsilent