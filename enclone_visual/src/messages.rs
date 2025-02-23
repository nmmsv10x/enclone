// Copyright (c) 2021 10X Genomics, Inc. All rights reserved.

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged1(String),
    InputChanged2(String),
    InputChangedN(String, usize),
    SubmitButtonPressed(Result<(), String>),
    BackButtonPressed(Result<(), String>),
    ForwardButtonPressed(Result<(), String>),
    DelButtonPressed(Result<(), String>),
    ComputationDone(Result<(), String>),
    // EventOccurred(iced_native::Event),
    GraphicsCopyButtonPressed,
    GraphicsCopyButtonFlashed(Result<(), String>),
    CommandCopyButtonPressed,
    DoNothing,
    Exit,
    ClearButtonPressed,
    RunTests(Result<(), String>),
    Capture(Result<(), String>),
    GroupClicked(crate::canvas_view::Message),
    Resize(u32, u32),
    HelpOpen(Result<(), String>),
    HelpClose(Result<(), String>),
    CookbookOpen,
    CookbookClose,
    CommandOpen(Result<(), String>),
    CommandClose,
    SummaryOpen(Result<(), String>),
    SummaryClose(Result<(), String>),
    ClonotypesOpen(Result<(), String>),
    ClonotypesClose,
    GraphicOpen(Result<(), String>),
    GraphicClose,
    ConsoleOpen,
    ConsoleClose,
    ArchiveOpen(Result<(), String>),
    ArchiveClose,
    ArchiveSaveClose,
    Save,
    SaveAs(String),
    CompleteSave(Result<(), String>),
    SaveOnExit,
    Restore(bool, usize),
    RestoreCookbook(bool, usize),
    ExpandArchiveEntry(bool, usize),
    ExpandCookbookEntry(bool, usize),
    DeleteArchiveEntry(bool, usize),
    ArchiveName(String, usize),
    ArchiveNameChange(usize),
    CompleteArchiveNameChange(Result<(), String>),
    Name(String),
    NameChange(bool),
    ArchiveShare(bool, usize),
    UserSelected(bool, usize),
    UserName(String, usize),
    DoShare(bool),
    CompleteDoShare(Result<(), String>),
    ArchiveRefresh,
    ArchiveRefreshComplete(Result<(), String>),
    OpenArchiveDoc,
    CloseArchiveDoc,
    OpenAlluvialReadsDoc,
    CloseAlluvialReadsDoc,
    ArchiveNarrative(usize),
    CopyArchiveNarrative(usize),
    CompleteCopyArchiveNarrative(Result<(), String>),
    CopyCookbookNarrative(usize),
    CompleteCopyCookbookNarrative(Result<(), String>),
    Narrative,
    Meta(Result<(), String>),
    CompleteMeta(Result<(), String>),
    NullMeta(Result<(), String>),
    Snap(&'static str),
    SetName(&'static str),
    CopyNarrative,
    CompleteCopyNarrative(Result<(), String>),
    CopySummary,
    CompleteCopySummary(Result<(), String>),
    WaitCommand(Result<(), String>),
    MetricButton(usize),
    CondenseMetrics,
    CopySelectedMetrics,
    CompleteCopySelectedMetrics(Result<(), String>),
    Snapshot,
    CompleteSnapshot(Result<(), String>),
    GraphicSnapshot,
    CompleteGraphicSnapshot(Result<(), String>),
    CommandSnapshot,
    CompleteCommandSnapshot(Result<(), String>),
    ClonotypesSnapshot,
    CompleteClonotypesSnapshot(Result<(), String>),
    SummarySnapshot,
    CompleteSummarySnapshot(Result<(), String>),
    Recompute,
    CopyLastNarrative,
    Sleep(u64),
    SanityCheck,
    CompleteSanityCheck(Result<(), String>),
    ClonotypesCopy,
    CompleteClonotypesCopy(Result<(), String>),
    TooltipToggle,
    CompleteTooltipToggle(Result<(), String>),
    CopyAlluvialTables,
    CompleteCopyAlluvialTables(Result<(), String>),
    CopyAlluvialReadsTables,
    CompleteCopyAlluvialReadsTables(Result<(), String>),
    SetSummaryScrollablePos(f32),
    CopyDescrips,
    CompleteCopyDescrips(Result<(), String>),
    GraphicPng,
    CompleteGraphicPng(Result<(), String>),
    GraphicHelp,
}
