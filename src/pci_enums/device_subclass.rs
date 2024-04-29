
// This file is AUTOGENERATED.
// Modify `tools/gen_classes.ers` if changes are needed to
// PCI device classes or the file format

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "pci_subclass_debug_strings", derive(Debug))]
/// Represent a PCI device subclass as an enumeration, for easier
/// matching with known valid values.
///
/// See PCI documentation or <https://wiki.osdev.org/PCI#Class_Codes>
/// for possible values.
pub enum PciDeviceSubclass {
    /// Enumeration matching device class 00h, subclass 00h.
    Unclassified_Generic,
    /// Enumeration matching device class 00h, subclass 01h.
    Unclassified_VgaCompatible,
    /// Enumeration matching unknown subclass values of device class 00h.
    Unclassified_Unknown(u8),
    /// Enumeration matching device class 01h, subclass 00h.
    MassStorageController_Scsi,
    /// Enumeration matching device class 01h, subclass 01h.
    MassStorageController_Ide,
    /// Enumeration matching device class 01h, subclass 02h.
    MassStorageController_FloppyDisk,
    /// Enumeration matching device class 01h, subclass 03h.
    MassStorageController_IpiBus,
    /// Enumeration matching device class 01h, subclass 04h.
    MassStorageController_Raid,
    /// Enumeration matching device class 01h, subclass 05h.
    MassStorageController_AtaWithAdma,
    /// Enumeration matching device class 01h, subclass 06h.
    MassStorageController_SerialAta,
    /// Enumeration matching device class 01h, subclass 07h.
    MassStorageController_SerialAttachedScsi,
    /// Enumeration matching device class 01h, subclass 08h.
    MassStorageController_NonVolatileMemory,
    /// Enumeration matching device class 01h, subclass 09h.
    MassStorageController_UniversalFlashStorage,
    /// Enumeration matching device class 01h, subclass 80h.
    MassStorageController_Other,
    /// Enumeration matching unknown subclass values of device class 01h.
    MassStorageController_Unknown(u8),
    /// Enumeration matching device class 02h, subclass 00h.
    NetworkController_Ethernet,
    /// Enumeration matching device class 02h, subclass 01h.
    NetworkController_TokenRing,
    /// Enumeration matching device class 02h, subclass 02h.
    NetworkController_Fddi,
    /// Enumeration matching device class 02h, subclass 03h.
    NetworkController_Atm,
    /// Enumeration matching device class 02h, subclass 04h.
    NetworkController_Isdn,
    /// Enumeration matching device class 02h, subclass 05h.
    NetworkController_WorldFip,
    /// Enumeration matching device class 02h, subclass 06h.
    NetworkController_PicMg214MultiComputing,
    /// Enumeration matching device class 02h, subclass 07h.
    NetworkController_InfiniBand,
    /// Enumeration matching device class 02h, subclass 80h.
    NetworkController_Other,
    /// Enumeration matching unknown subclass values of device class 02h.
    NetworkController_Unknown(u8),
    /// Enumeration matching device class 03h, subclass 00h.
    DisplayController_VgaCompatible,
    /// Enumeration matching device class 03h, subclass 01h.
    DisplayController_Xga,
    /// Enumeration matching device class 03h, subclass 02h.
    DisplayController_NonVga3D,
    /// Enumeration matching device class 03h, subclass 80h.
    DisplayController_Other,
    /// Enumeration matching unknown subclass values of device class 03h.
    DisplayController_Unknown(u8),
    /// Enumeration matching device class 04h, subclass 00h.
    MultimediaDevice_VideoDevice,
    /// Enumeration matching device class 04h, subclass 01h.
    MultimediaDevice_AudioDevice,
    /// Enumeration matching device class 04h, subclass 02h.
    MultimediaDevice_ComputerTelephony,
    /// Enumeration matching device class 04h, subclass 03h.
    MultimediaDevice_HdaCompatible,
    /// Enumeration matching device class 04h, subclass 80h.
    MultimediaDevice_Other,
    /// Enumeration matching unknown subclass values of device class 04h.
    MultimediaDevice_Unknown(u8),
    /// Enumeration matching device class 05h, subclass 00h.
    MemoryController_Ram,
    /// Enumeration matching device class 05h, subclass 01h.
    MemoryController_Flash,
    /// Enumeration matching device class 05h, subclass 02h.
    MemoryController_CxlMemory,
    /// Enumeration matching device class 05h, subclass 80h.
    MemoryController_Other,
    /// Enumeration matching unknown subclass values of device class 05h.
    MemoryController_Unknown(u8),
    /// Enumeration matching device class 06h, subclass 00h.
    Bridge_HostBridge,
    /// Enumeration matching device class 06h, subclass 01h.
    Bridge_IsaBridge,
    /// Enumeration matching device class 06h, subclass 02h.
    Bridge_EisaBridge,
    /// Enumeration matching device class 06h, subclass 03h.
    Bridge_McaBridge,
    /// Enumeration matching device class 06h, subclass 04h.
    Bridge_PciToPciBridge,
    /// Enumeration matching device class 06h, subclass 05h.
    Bridge_PcmciaBridge,
    /// Enumeration matching device class 06h, subclass 06h.
    Bridge_NuBusBridge,
    /// Enumeration matching device class 06h, subclass 07h.
    Bridge_CardBusBridge,
    /// Enumeration matching device class 06h, subclass 08h.
    Bridge_RacewayBridge,
    /// Enumeration matching device class 06h, subclass 09h.
    Bridge_SemiTransparentPciToPciBridge,
    /// Enumeration matching device class 06h, subclass 0Ah.
    Bridge_InfiniBandtoPciHostBridge,
    /// Enumeration matching device class 06h, subclass 0Bh.
    Bridge_AdvancedSwitchingToPciHostBridge,
    /// Enumeration matching device class 06h, subclass 80h.
    Bridge_Other,
    /// Enumeration matching unknown subclass values of device class 06h.
    Bridge_Unknown(u8),
    /// Enumeration matching device class 07h, subclass 00h.
    CommunicationController_Serial,
    /// Enumeration matching device class 07h, subclass 01h.
    CommunicationController_Parallel,
    /// Enumeration matching device class 07h, subclass 02h.
    CommunicationController_MultiportSerialController,
    /// Enumeration matching device class 07h, subclass 03h.
    CommunicationController_Modem,
    /// Enumeration matching device class 07h, subclass 04h.
    CommunicationController_GpibController,
    /// Enumeration matching device class 07h, subclass 05h.
    CommunicationController_Smartcard,
    /// Enumeration matching device class 07h, subclass 80h.
    CommunicationController_Other,
    /// Enumeration matching unknown subclass values of device class 07h.
    CommunicationController_Unknown(u8),
    /// Enumeration matching device class 08h, subclass 00h.
    BaseSystemPeripheral_PIC,
    /// Enumeration matching device class 08h, subclass 01h.
    BaseSystemPeripheral_DmaController,
    /// Enumeration matching device class 08h, subclass 02h.
    BaseSystemPeripheral_SystemTimer,
    /// Enumeration matching device class 08h, subclass 03h.
    BaseSystemPeripheral_RealTimeClockController,
    /// Enumeration matching device class 08h, subclass 04h.
    BaseSystemPeripheral_GenericPciHotPlugController,
    /// Enumeration matching device class 08h, subclass 05h.
    BaseSystemPeripheral_SdHostController,
    /// Enumeration matching device class 08h, subclass 06h.
    BaseSystemPeripheral_IOMMU,
    /// Enumeration matching device class 08h, subclass 07h.
    BaseSystemPeripheral_RootComplexEventCollector,
    /// Enumeration matching device class 08h, subclass 80h.
    BaseSystemPeripheral_Other,
    /// Enumeration matching unknown subclass values of device class 08h.
    BaseSystemPeripheral_Unknown(u8),
    /// Enumeration matching device class 09h, subclass 00h.
    InputDevice_KeyboardController,
    /// Enumeration matching device class 09h, subclass 01h.
    InputDevice_DigitizerPenController,
    /// Enumeration matching device class 09h, subclass 02h.
    InputDevice_MouseController,
    /// Enumeration matching device class 09h, subclass 03h.
    InputDevice_ScannerController,
    /// Enumeration matching device class 09h, subclass 04h.
    InputDevice_GameportController,
    /// Enumeration matching device class 09h, subclass 80h.
    InputDevice_Other,
    /// Enumeration matching unknown subclass values of device class 09h.
    InputDevice_Unknown(u8),
    /// Enumeration matching device class 0Ah, subclass 00h.
    DockingStation_Generic,
    /// Enumeration matching device class 0Ah, subclass 80h.
    DockingStation_Other,
    /// Enumeration matching unknown subclass values of device class 0Ah.
    DockingStation_Unknown(u8),
    /// Enumeration matching device class 0Bh, subclass 00h.
    Processor_Intel386,
    /// Enumeration matching device class 0Bh, subclass 01h.
    Processor_Intel486,
    /// Enumeration matching device class 0Bh, subclass 02h.
    Processor_IntelPentium,
    /// Enumeration matching device class 0Bh, subclass 10h.
    Processor_DecAlpha,
    /// Enumeration matching device class 0Bh, subclass 20h.
    Processor_PowerPc,
    /// Enumeration matching device class 0Bh, subclass 30h.
    Processor_Mips,
    /// Enumeration matching device class 0Bh, subclass 40h.
    Processor_Coprocessor,
    /// Enumeration matching device class 0Bh, subclass 80h.
    Processor_Other,
    /// Enumeration matching unknown subclass values of device class 0Bh.
    Processor_Unknown(u8),
    /// Enumeration matching device class 0Ch, subclass 00h.
    SerialBusController_Ieee1394,
    /// Enumeration matching device class 0Ch, subclass 01h.
    SerialBusController_ACCESSbus,
    /// Enumeration matching device class 0Ch, subclass 02h.
    SerialBusController_SSA,
    /// Enumeration matching device class 0Ch, subclass 03h.
    SerialBusController_USB,
    /// Enumeration matching device class 0Ch, subclass 04h.
    SerialBusController_FibreChannel,
    /// Enumeration matching device class 0Ch, subclass 05h.
    SerialBusController_SystemManagementBus,
    /// Enumeration matching device class 0Ch, subclass 06h.
    SerialBusController_InfiniBandDeprecated,
    /// Enumeration matching device class 0Ch, subclass 07h.
    SerialBusController_Ipmi,
    /// Enumeration matching device class 0Ch, subclass 08h.
    SerialBusController_SERCOS,
    /// Enumeration matching device class 0Ch, subclass 09h.
    SerialBusController_CANbus,
    /// Enumeration matching device class 0Ch, subclass 0Ah.
    SerialBusController_MipiI3C,
    /// Enumeration matching device class 0Ch, subclass 80h.
    SerialBusController_Other,
    /// Enumeration matching unknown subclass values of device class 0Ch.
    SerialBusController_Unknown(u8),
    /// Enumeration matching device class 0Dh, subclass 00h.
    WirelessController_iRDA,
    /// Enumeration matching device class 0Dh, subclass 01h.
    WirelessController_Radio,
    /// Enumeration matching device class 0Dh, subclass 10h.
    WirelessController_RFController,
    /// Enumeration matching device class 0Dh, subclass 11h.
    WirelessController_Bluetooth,
    /// Enumeration matching device class 0Dh, subclass 12h.
    WirelessController_Broadband,
    /// Enumeration matching device class 0Dh, subclass 20h.
    WirelessController_Wifi802_11A,
    /// Enumeration matching device class 0Dh, subclass 21h.
    WirelessController_Wifi802_11B,
    /// Enumeration matching device class 0Dh, subclass 40h.
    WirelessController_CellularController,
    /// Enumeration matching device class 0Dh, subclass 41h.
    WirelessController_CellularControllerWithEthernet,
    /// Enumeration matching device class 0Dh, subclass 80h.
    WirelessController_Other,
    /// Enumeration matching unknown subclass values of device class 0Dh.
    WirelessController_Unknown(u8),
    /// Enumeration matching device class 0Eh, subclass 00h.
    IntelligentIoController_IntelligentIoController,
    /// Enumeration matching unknown subclass values of device class 0Eh.
    IntelligentIoController_Unknown(u8),
    /// Enumeration matching device class 0Fh, subclass 01h.
    SatelliteCommController_Tv,
    /// Enumeration matching device class 0Fh, subclass 02h.
    SatelliteCommController_Audio,
    /// Enumeration matching device class 0Fh, subclass 03h.
    SatelliteCommController_Voice,
    /// Enumeration matching device class 0Fh, subclass 04h.
    SatelliteCommController_Data,
    /// Enumeration matching device class 0Fh, subclass 80h.
    SatelliteCommController_Other,
    /// Enumeration matching unknown subclass values of device class 0Fh.
    SatelliteCommController_Unknown(u8),
    /// Enumeration matching device class 10h, subclass 00h.
    EncryptionController_NetworkandComputingEncrpytionDecryption,
    /// Enumeration matching device class 10h, subclass 10h.
    EncryptionController_EntertainmentEncryptionDecryption,
    /// Enumeration matching device class 10h, subclass 80h.
    EncryptionController_Other,
    /// Enumeration matching unknown subclass values of device class 10h.
    EncryptionController_Unknown(u8),
    /// Enumeration matching device class 11h, subclass 00h.
    SignalProcessingController_DpIoModule,
    /// Enumeration matching device class 11h, subclass 01h.
    SignalProcessingController_PerformanceCounters,
    /// Enumeration matching device class 11h, subclass 10h.
    SignalProcessingController_CommunicationSynchronization,
    /// Enumeration matching device class 11h, subclass 20h.
    SignalProcessingController_ManagementCard,
    /// Enumeration matching device class 11h, subclass 80h.
    SignalProcessingController_Other,
    /// Enumeration matching unknown subclass values of device class 11h.
    SignalProcessingController_Unknown(u8),
    /// Enumeration matching device class 12h, subclass 00h.
    ProcessingAccelerator_VendorSpecific,
    /// Enumeration matching device class 12h, subclass 01h.
    ProcessingAccelerator_SniaSdxiController,
    /// Enumeration matching unknown subclass values of device class 12h.
    ProcessingAccelerator_Unknown(u8),
    /// Enumeration matching device class 13h, subclass 00h.
    NonEssentialInstrumentation_VendorSpecific,
    /// Enumeration matching unknown subclass values of device class 13h.
    NonEssentialInstrumentation_Unknown(u8),
    /// Enumeration matching unknown combinations of device class / subclass.
    Unknown(u8, u8),
}

impl PciDeviceSubclass {
    /// Converts a combination of `class_code` and `subclass_code`
    /// to a valid `PciDeviceSubclass` object.
    pub fn from_codes(class_code: u8, subclass_code: u8) -> Self {
        match (class_code, subclass_code) {
            (0x00, 0x00) => Self::Unclassified_Generic,
            (0x00, 0x01) => Self::Unclassified_VgaCompatible,
            (0x00, unk_s) => Self::Unclassified_Unknown(unk_s),
            (0x01, 0x00) => Self::MassStorageController_Scsi,
            (0x01, 0x01) => Self::MassStorageController_Ide,
            (0x01, 0x02) => Self::MassStorageController_FloppyDisk,
            (0x01, 0x03) => Self::MassStorageController_IpiBus,
            (0x01, 0x04) => Self::MassStorageController_Raid,
            (0x01, 0x05) => Self::MassStorageController_AtaWithAdma,
            (0x01, 0x06) => Self::MassStorageController_SerialAta,
            (0x01, 0x07) => Self::MassStorageController_SerialAttachedScsi,
            (0x01, 0x08) => Self::MassStorageController_NonVolatileMemory,
            (0x01, 0x09) => Self::MassStorageController_UniversalFlashStorage,
            (0x01, 0x80) => Self::MassStorageController_Other,
            (0x01, unk_s) => Self::MassStorageController_Unknown(unk_s),
            (0x02, 0x00) => Self::NetworkController_Ethernet,
            (0x02, 0x01) => Self::NetworkController_TokenRing,
            (0x02, 0x02) => Self::NetworkController_Fddi,
            (0x02, 0x03) => Self::NetworkController_Atm,
            (0x02, 0x04) => Self::NetworkController_Isdn,
            (0x02, 0x05) => Self::NetworkController_WorldFip,
            (0x02, 0x06) => Self::NetworkController_PicMg214MultiComputing,
            (0x02, 0x07) => Self::NetworkController_InfiniBand,
            (0x02, 0x80) => Self::NetworkController_Other,
            (0x02, unk_s) => Self::NetworkController_Unknown(unk_s),
            (0x03, 0x00) => Self::DisplayController_VgaCompatible,
            (0x03, 0x01) => Self::DisplayController_Xga,
            (0x03, 0x02) => Self::DisplayController_NonVga3D,
            (0x03, 0x80) => Self::DisplayController_Other,
            (0x03, unk_s) => Self::DisplayController_Unknown(unk_s),
            (0x04, 0x00) => Self::MultimediaDevice_VideoDevice,
            (0x04, 0x01) => Self::MultimediaDevice_AudioDevice,
            (0x04, 0x02) => Self::MultimediaDevice_ComputerTelephony,
            (0x04, 0x03) => Self::MultimediaDevice_HdaCompatible,
            (0x04, 0x80) => Self::MultimediaDevice_Other,
            (0x04, unk_s) => Self::MultimediaDevice_Unknown(unk_s),
            (0x05, 0x00) => Self::MemoryController_Ram,
            (0x05, 0x01) => Self::MemoryController_Flash,
            (0x05, 0x02) => Self::MemoryController_CxlMemory,
            (0x05, 0x80) => Self::MemoryController_Other,
            (0x05, unk_s) => Self::MemoryController_Unknown(unk_s),
            (0x06, 0x00) => Self::Bridge_HostBridge,
            (0x06, 0x01) => Self::Bridge_IsaBridge,
            (0x06, 0x02) => Self::Bridge_EisaBridge,
            (0x06, 0x03) => Self::Bridge_McaBridge,
            (0x06, 0x04) => Self::Bridge_PciToPciBridge,
            (0x06, 0x05) => Self::Bridge_PcmciaBridge,
            (0x06, 0x06) => Self::Bridge_NuBusBridge,
            (0x06, 0x07) => Self::Bridge_CardBusBridge,
            (0x06, 0x08) => Self::Bridge_RacewayBridge,
            (0x06, 0x09) => Self::Bridge_SemiTransparentPciToPciBridge,
            (0x06, 0x0a) => Self::Bridge_InfiniBandtoPciHostBridge,
            (0x06, 0x0b) => Self::Bridge_AdvancedSwitchingToPciHostBridge,
            (0x06, 0x80) => Self::Bridge_Other,
            (0x06, unk_s) => Self::Bridge_Unknown(unk_s),
            (0x07, 0x00) => Self::CommunicationController_Serial,
            (0x07, 0x01) => Self::CommunicationController_Parallel,
            (0x07, 0x02) => Self::CommunicationController_MultiportSerialController,
            (0x07, 0x03) => Self::CommunicationController_Modem,
            (0x07, 0x04) => Self::CommunicationController_GpibController,
            (0x07, 0x05) => Self::CommunicationController_Smartcard,
            (0x07, 0x80) => Self::CommunicationController_Other,
            (0x07, unk_s) => Self::CommunicationController_Unknown(unk_s),
            (0x08, 0x00) => Self::BaseSystemPeripheral_PIC,
            (0x08, 0x01) => Self::BaseSystemPeripheral_DmaController,
            (0x08, 0x02) => Self::BaseSystemPeripheral_SystemTimer,
            (0x08, 0x03) => Self::BaseSystemPeripheral_RealTimeClockController,
            (0x08, 0x04) => Self::BaseSystemPeripheral_GenericPciHotPlugController,
            (0x08, 0x05) => Self::BaseSystemPeripheral_SdHostController,
            (0x08, 0x06) => Self::BaseSystemPeripheral_IOMMU,
            (0x08, 0x07) => Self::BaseSystemPeripheral_RootComplexEventCollector,
            (0x08, 0x80) => Self::BaseSystemPeripheral_Other,
            (0x08, unk_s) => Self::BaseSystemPeripheral_Unknown(unk_s),
            (0x09, 0x00) => Self::InputDevice_KeyboardController,
            (0x09, 0x01) => Self::InputDevice_DigitizerPenController,
            (0x09, 0x02) => Self::InputDevice_MouseController,
            (0x09, 0x03) => Self::InputDevice_ScannerController,
            (0x09, 0x04) => Self::InputDevice_GameportController,
            (0x09, 0x80) => Self::InputDevice_Other,
            (0x09, unk_s) => Self::InputDevice_Unknown(unk_s),
            (0x0a, 0x00) => Self::DockingStation_Generic,
            (0x0a, 0x80) => Self::DockingStation_Other,
            (0x0a, unk_s) => Self::DockingStation_Unknown(unk_s),
            (0x0b, 0x00) => Self::Processor_Intel386,
            (0x0b, 0x01) => Self::Processor_Intel486,
            (0x0b, 0x02) => Self::Processor_IntelPentium,
            (0x0b, 0x10) => Self::Processor_DecAlpha,
            (0x0b, 0x20) => Self::Processor_PowerPc,
            (0x0b, 0x30) => Self::Processor_Mips,
            (0x0b, 0x40) => Self::Processor_Coprocessor,
            (0x0b, 0x80) => Self::Processor_Other,
            (0x0b, unk_s) => Self::Processor_Unknown(unk_s),
            (0x0c, 0x00) => Self::SerialBusController_Ieee1394,
            (0x0c, 0x01) => Self::SerialBusController_ACCESSbus,
            (0x0c, 0x02) => Self::SerialBusController_SSA,
            (0x0c, 0x03) => Self::SerialBusController_USB,
            (0x0c, 0x04) => Self::SerialBusController_FibreChannel,
            (0x0c, 0x05) => Self::SerialBusController_SystemManagementBus,
            (0x0c, 0x06) => Self::SerialBusController_InfiniBandDeprecated,
            (0x0c, 0x07) => Self::SerialBusController_Ipmi,
            (0x0c, 0x08) => Self::SerialBusController_SERCOS,
            (0x0c, 0x09) => Self::SerialBusController_CANbus,
            (0x0c, 0x0a) => Self::SerialBusController_MipiI3C,
            (0x0c, 0x80) => Self::SerialBusController_Other,
            (0x0c, unk_s) => Self::SerialBusController_Unknown(unk_s),
            (0x0d, 0x00) => Self::WirelessController_iRDA,
            (0x0d, 0x01) => Self::WirelessController_Radio,
            (0x0d, 0x10) => Self::WirelessController_RFController,
            (0x0d, 0x11) => Self::WirelessController_Bluetooth,
            (0x0d, 0x12) => Self::WirelessController_Broadband,
            (0x0d, 0x20) => Self::WirelessController_Wifi802_11A,
            (0x0d, 0x21) => Self::WirelessController_Wifi802_11B,
            (0x0d, 0x40) => Self::WirelessController_CellularController,
            (0x0d, 0x41) => Self::WirelessController_CellularControllerWithEthernet,
            (0x0d, 0x80) => Self::WirelessController_Other,
            (0x0d, unk_s) => Self::WirelessController_Unknown(unk_s),
            (0x0e, 0x00) => Self::IntelligentIoController_IntelligentIoController,
            (0x0e, unk_s) => Self::IntelligentIoController_Unknown(unk_s),
            (0x0f, 0x01) => Self::SatelliteCommController_Tv,
            (0x0f, 0x02) => Self::SatelliteCommController_Audio,
            (0x0f, 0x03) => Self::SatelliteCommController_Voice,
            (0x0f, 0x04) => Self::SatelliteCommController_Data,
            (0x0f, 0x80) => Self::SatelliteCommController_Other,
            (0x0f, unk_s) => Self::SatelliteCommController_Unknown(unk_s),
            (0x10, 0x00) => Self::EncryptionController_NetworkandComputingEncrpytionDecryption,
            (0x10, 0x10) => Self::EncryptionController_EntertainmentEncryptionDecryption,
            (0x10, 0x80) => Self::EncryptionController_Other,
            (0x10, unk_s) => Self::EncryptionController_Unknown(unk_s),
            (0x11, 0x00) => Self::SignalProcessingController_DpIoModule,
            (0x11, 0x01) => Self::SignalProcessingController_PerformanceCounters,
            (0x11, 0x10) => Self::SignalProcessingController_CommunicationSynchronization,
            (0x11, 0x20) => Self::SignalProcessingController_ManagementCard,
            (0x11, 0x80) => Self::SignalProcessingController_Other,
            (0x11, unk_s) => Self::SignalProcessingController_Unknown(unk_s),
            (0x12, 0x00) => Self::ProcessingAccelerator_VendorSpecific,
            (0x12, 0x01) => Self::ProcessingAccelerator_SniaSdxiController,
            (0x12, unk_s) => Self::ProcessingAccelerator_Unknown(unk_s),
            (0x13, 0x00) => Self::NonEssentialInstrumentation_VendorSpecific,
            (0x13, unk_s) => Self::NonEssentialInstrumentation_Unknown(unk_s),
            (unk_c, unk_s) => Self::Unknown(unk_c, unk_s),
        }
    }

    /// Converts a `PciDeviceSubclass` object into the
    /// `class_code` and `subclass_code` that it represents.
    pub fn as_codes(&self) -> (u8, u8) {
        match self {
            Self::Unclassified_Generic => (0x00, 0x00),
            Self::Unclassified_VgaCompatible => (0x00, 0x01),
            Self::Unclassified_Unknown(unk) => (0x00, *unk),
            Self::MassStorageController_Scsi => (0x01, 0x00),
            Self::MassStorageController_Ide => (0x01, 0x01),
            Self::MassStorageController_FloppyDisk => (0x01, 0x02),
            Self::MassStorageController_IpiBus => (0x01, 0x03),
            Self::MassStorageController_Raid => (0x01, 0x04),
            Self::MassStorageController_AtaWithAdma => (0x01, 0x05),
            Self::MassStorageController_SerialAta => (0x01, 0x06),
            Self::MassStorageController_SerialAttachedScsi => (0x01, 0x07),
            Self::MassStorageController_NonVolatileMemory => (0x01, 0x08),
            Self::MassStorageController_UniversalFlashStorage => (0x01, 0x09),
            Self::MassStorageController_Other => (0x01, 0x80),
            Self::MassStorageController_Unknown(unk) => (0x01, *unk),
            Self::NetworkController_Ethernet => (0x02, 0x00),
            Self::NetworkController_TokenRing => (0x02, 0x01),
            Self::NetworkController_Fddi => (0x02, 0x02),
            Self::NetworkController_Atm => (0x02, 0x03),
            Self::NetworkController_Isdn => (0x02, 0x04),
            Self::NetworkController_WorldFip => (0x02, 0x05),
            Self::NetworkController_PicMg214MultiComputing => (0x02, 0x06),
            Self::NetworkController_InfiniBand => (0x02, 0x07),
            Self::NetworkController_Other => (0x02, 0x80),
            Self::NetworkController_Unknown(unk) => (0x02, *unk),
            Self::DisplayController_VgaCompatible => (0x03, 0x00),
            Self::DisplayController_Xga => (0x03, 0x01),
            Self::DisplayController_NonVga3D => (0x03, 0x02),
            Self::DisplayController_Other => (0x03, 0x80),
            Self::DisplayController_Unknown(unk) => (0x03, *unk),
            Self::MultimediaDevice_VideoDevice => (0x04, 0x00),
            Self::MultimediaDevice_AudioDevice => (0x04, 0x01),
            Self::MultimediaDevice_ComputerTelephony => (0x04, 0x02),
            Self::MultimediaDevice_HdaCompatible => (0x04, 0x03),
            Self::MultimediaDevice_Other => (0x04, 0x80),
            Self::MultimediaDevice_Unknown(unk) => (0x04, *unk),
            Self::MemoryController_Ram => (0x05, 0x00),
            Self::MemoryController_Flash => (0x05, 0x01),
            Self::MemoryController_CxlMemory => (0x05, 0x02),
            Self::MemoryController_Other => (0x05, 0x80),
            Self::MemoryController_Unknown(unk) => (0x05, *unk),
            Self::Bridge_HostBridge => (0x06, 0x00),
            Self::Bridge_IsaBridge => (0x06, 0x01),
            Self::Bridge_EisaBridge => (0x06, 0x02),
            Self::Bridge_McaBridge => (0x06, 0x03),
            Self::Bridge_PciToPciBridge => (0x06, 0x04),
            Self::Bridge_PcmciaBridge => (0x06, 0x05),
            Self::Bridge_NuBusBridge => (0x06, 0x06),
            Self::Bridge_CardBusBridge => (0x06, 0x07),
            Self::Bridge_RacewayBridge => (0x06, 0x08),
            Self::Bridge_SemiTransparentPciToPciBridge => (0x06, 0x09),
            Self::Bridge_InfiniBandtoPciHostBridge => (0x06, 0x0a),
            Self::Bridge_AdvancedSwitchingToPciHostBridge => (0x06, 0x0b),
            Self::Bridge_Other => (0x06, 0x80),
            Self::Bridge_Unknown(unk) => (0x06, *unk),
            Self::CommunicationController_Serial => (0x07, 0x00),
            Self::CommunicationController_Parallel => (0x07, 0x01),
            Self::CommunicationController_MultiportSerialController => (0x07, 0x02),
            Self::CommunicationController_Modem => (0x07, 0x03),
            Self::CommunicationController_GpibController => (0x07, 0x04),
            Self::CommunicationController_Smartcard => (0x07, 0x05),
            Self::CommunicationController_Other => (0x07, 0x80),
            Self::CommunicationController_Unknown(unk) => (0x07, *unk),
            Self::BaseSystemPeripheral_PIC => (0x08, 0x00),
            Self::BaseSystemPeripheral_DmaController => (0x08, 0x01),
            Self::BaseSystemPeripheral_SystemTimer => (0x08, 0x02),
            Self::BaseSystemPeripheral_RealTimeClockController => (0x08, 0x03),
            Self::BaseSystemPeripheral_GenericPciHotPlugController => (0x08, 0x04),
            Self::BaseSystemPeripheral_SdHostController => (0x08, 0x05),
            Self::BaseSystemPeripheral_IOMMU => (0x08, 0x06),
            Self::BaseSystemPeripheral_RootComplexEventCollector => (0x08, 0x07),
            Self::BaseSystemPeripheral_Other => (0x08, 0x80),
            Self::BaseSystemPeripheral_Unknown(unk) => (0x08, *unk),
            Self::InputDevice_KeyboardController => (0x09, 0x00),
            Self::InputDevice_DigitizerPenController => (0x09, 0x01),
            Self::InputDevice_MouseController => (0x09, 0x02),
            Self::InputDevice_ScannerController => (0x09, 0x03),
            Self::InputDevice_GameportController => (0x09, 0x04),
            Self::InputDevice_Other => (0x09, 0x80),
            Self::InputDevice_Unknown(unk) => (0x09, *unk),
            Self::DockingStation_Generic => (0x0a, 0x00),
            Self::DockingStation_Other => (0x0a, 0x80),
            Self::DockingStation_Unknown(unk) => (0x0a, *unk),
            Self::Processor_Intel386 => (0x0b, 0x00),
            Self::Processor_Intel486 => (0x0b, 0x01),
            Self::Processor_IntelPentium => (0x0b, 0x02),
            Self::Processor_DecAlpha => (0x0b, 0x10),
            Self::Processor_PowerPc => (0x0b, 0x20),
            Self::Processor_Mips => (0x0b, 0x30),
            Self::Processor_Coprocessor => (0x0b, 0x40),
            Self::Processor_Other => (0x0b, 0x80),
            Self::Processor_Unknown(unk) => (0x0b, *unk),
            Self::SerialBusController_Ieee1394 => (0x0c, 0x00),
            Self::SerialBusController_ACCESSbus => (0x0c, 0x01),
            Self::SerialBusController_SSA => (0x0c, 0x02),
            Self::SerialBusController_USB => (0x0c, 0x03),
            Self::SerialBusController_FibreChannel => (0x0c, 0x04),
            Self::SerialBusController_SystemManagementBus => (0x0c, 0x05),
            Self::SerialBusController_InfiniBandDeprecated => (0x0c, 0x06),
            Self::SerialBusController_Ipmi => (0x0c, 0x07),
            Self::SerialBusController_SERCOS => (0x0c, 0x08),
            Self::SerialBusController_CANbus => (0x0c, 0x09),
            Self::SerialBusController_MipiI3C => (0x0c, 0x0a),
            Self::SerialBusController_Other => (0x0c, 0x80),
            Self::SerialBusController_Unknown(unk) => (0x0c, *unk),
            Self::WirelessController_iRDA => (0x0d, 0x00),
            Self::WirelessController_Radio => (0x0d, 0x01),
            Self::WirelessController_RFController => (0x0d, 0x10),
            Self::WirelessController_Bluetooth => (0x0d, 0x11),
            Self::WirelessController_Broadband => (0x0d, 0x12),
            Self::WirelessController_Wifi802_11A => (0x0d, 0x20),
            Self::WirelessController_Wifi802_11B => (0x0d, 0x21),
            Self::WirelessController_CellularController => (0x0d, 0x40),
            Self::WirelessController_CellularControllerWithEthernet => (0x0d, 0x41),
            Self::WirelessController_Other => (0x0d, 0x80),
            Self::WirelessController_Unknown(unk) => (0x0d, *unk),
            Self::IntelligentIoController_IntelligentIoController => (0x0e, 0x00),
            Self::IntelligentIoController_Unknown(unk) => (0x0e, *unk),
            Self::SatelliteCommController_Tv => (0x0f, 0x01),
            Self::SatelliteCommController_Audio => (0x0f, 0x02),
            Self::SatelliteCommController_Voice => (0x0f, 0x03),
            Self::SatelliteCommController_Data => (0x0f, 0x04),
            Self::SatelliteCommController_Other => (0x0f, 0x80),
            Self::SatelliteCommController_Unknown(unk) => (0x0f, *unk),
            Self::EncryptionController_NetworkandComputingEncrpytionDecryption => (0x10, 0x00),
            Self::EncryptionController_EntertainmentEncryptionDecryption => (0x10, 0x10),
            Self::EncryptionController_Other => (0x10, 0x80),
            Self::EncryptionController_Unknown(unk) => (0x10, *unk),
            Self::SignalProcessingController_DpIoModule => (0x11, 0x00),
            Self::SignalProcessingController_PerformanceCounters => (0x11, 0x01),
            Self::SignalProcessingController_CommunicationSynchronization => (0x11, 0x10),
            Self::SignalProcessingController_ManagementCard => (0x11, 0x20),
            Self::SignalProcessingController_Other => (0x11, 0x80),
            Self::SignalProcessingController_Unknown(unk) => (0x11, *unk),
            Self::ProcessingAccelerator_VendorSpecific => (0x12, 0x00),
            Self::ProcessingAccelerator_SniaSdxiController => (0x12, 0x01),
            Self::ProcessingAccelerator_Unknown(unk) => (0x12, *unk),
            Self::NonEssentialInstrumentation_VendorSpecific => (0x13, 0x00),
            Self::NonEssentialInstrumentation_Unknown(unk) => (0x13, *unk),
            Self::Unknown(unk_c, unk_s) => (*unk_c, *unk_s),
        }
    }
}

impl From<PciDeviceSubclass> for u8 {
    fn from(value: PciDeviceSubclass) -> Self {
        value.as_codes().1
    }
}

#[cfg(not(feature = "pci_subclass_debug_strings"))]
impl std::fmt::Debug for PciDeviceSubclass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.as_codes();
        write!(f, "(clss:{:02X}h subc:{:02X}h)", v.0, v.1)
    }
}
