#[doc = "Register `USB_CHEP1R` reader"]
pub type R = crate::R<UsbChep1rSpec>;
#[doc = "Register `USB_CHEP1R` writer"]
pub type W = crate::W<UsbChep1rSpec>;
#[doc = "Field `EA` reader - endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction."]
pub type EaR = crate::FieldReader;
#[doc = "Field `EA` writer - endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction."]
pub type EaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STATTX` writer - Status bits, for transmission transfers"]
pub type StattxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTOGTX` writer - Data toggle, for transmission transfers If the endpoint/channel is non-isochronous, this bit contains the required value of the data toggle bit (01=1DATA0, 11=1DATA1) for the next data packet to be transmitted. Hardware toggles this bit when the ACK handshake is received from the USB host, following a data packet transmission. If the endpoint/channel is defined as a control one, hardware sets this bit to 1 at the reception of a SETUP PID addressed to this endpoint (in device mode) or when a SETUP transaction is acknowledged by the device (in host mode). If the endpoint/channel is using the double buffer feature, this bit is used to support packet buffer swapping too (Refer to Section134.5.3: Double-buffered endpoints and usage in Device mode). If the endpoint/channel is isochronous, this bit is used to support packet buffer swapping since no data toggling is used for this sort of endpoints and only DATA0 packet are transmitted (refer to Section134.5.5: Isochronous transfers in Device mode). Hardware toggles this bit just after the end of data packet transmission, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint/channel is not a control one) or to force a specific data toggle/packet buffer usage. When the application software writes 0, the value of DTOGTX remains unchanged, while writing 1 makes the bit value to toggle. This bit is read/write but it can only be toggled by writing 1."]
pub type DtogtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTTX` reader - Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written. Host mode Same as VTRX behavior but for USB OUT and SETUP transactions."]
pub type VttxR = crate::BitReader;
#[doc = "Field `VTTX` writer - Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written. Host mode Same as VTRX behavior but for USB OUT and SETUP transactions."]
pub type VttxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPKIND` reader - endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the UTYPE bits. Table1217 summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Section134.5.3: Double-buffered endpoints and usage in Device mode. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered STALL instead of ACK. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required."]
pub type EpkindR = crate::BitReader;
#[doc = "Field `EPKIND` writer - endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the UTYPE bits. Table1217 summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Section134.5.3: Double-buffered endpoints and usage in Device mode. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered STALL instead of ACK. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required."]
pub type EpkindW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTYPE` reader - USB type of transaction These bits configure the behavior of this endpoint/channel as described in Table1216: Endpoint/channel type encoding. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral does not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet is accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of isochronous channels/endpoints is explained in Section134.5.5: Isochronous transfers in Device mode"]
pub type UtypeR = crate::FieldReader;
#[doc = "Field `UTYPE` writer - USB type of transaction These bits configure the behavior of this endpoint/channel as described in Table1216: Endpoint/channel type encoding. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral does not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet is accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of isochronous channels/endpoints is explained in Section134.5.5: Isochronous transfers in Device mode"]
pub type UtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SETUP` reader - Setup transaction completed Device mode This bit is read-only and it is set by the hardware when the last completed transaction is a SETUP. This bit changes its value only for control endpoints. It must be examined, in the case of a successful receive transaction (VTRX event), to determine the type of transaction occurred. To protect the interrupt service routine from the changes in SETUP bits due to next incoming tokens, this bit is kept frozen while VTRX bit is at 1; its state changes when VTRX is at 0. This bit is read-only. Host mode This bit is set by the software to send a SETUP transaction on a control endpoint. This bit changes its value only for control endpoints. It is cleared by hardware when the SETUP transaction is acknowledged and VTTX interrupt generated."]
pub type SetupR = crate::BitReader;
#[doc = "Field `STATRX` writer - Status bits, for reception transfers Device mode These bits contain information about the endpoint status, which are listed in Table1215: Reception status encoding on page11025. These bits can be toggled by software to initialize their value. When the application software writes 0, the value remains unchanged, while writing 1 makes the bit value to toggle. Hardware sets the STATRX bits to NAK when a correct transfer has occurred (VTRX1=11) corresponding to a OUT or SETUP (control only) transaction addressed to this endpoint, so the software has the time to elaborate the received data before it acknowledges a new transaction. Double-buffered bulk endpoints implement a special transaction flow control, which control the status based upon buffer availability condition (Refer to Section134.5.3: Double-buffered endpoints and usage in Device mode). If the endpoint is defined as isochronous, its status can be only VALID or DISABLED, so that the hardware cannot change the status of the endpoint after a successful transaction. If the software sets the STATRX bits to STALL or NAK for an isochronous endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing 1. Host mode These bits are the host application controls to start, retry, or abort host transactions driven by the channel. These bits also contain information about the device answer to the last IN channel transaction and report the current status of the channel according to the following STATRX table of states: - DISABLE DISABLE value is reported in case of ACK acknowledge is received on a single-buffer channel. When in DISABLE state the channel is unused or not active waiting for application to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a transaction. In this case the transaction is immediately removed from the host execution list. If the aborted transaction was already under execution it is regularly terminated on the USB but the relative VTRX interrupt is not generated. - VALID A host channel is actively trying to submit USB transaction to device only when in VALID state.VALID state can be set by software or automatically by hardware on a NAKED channel at the start of a new frame. When set to VALID, an host channel enters the host execution queue and waits permission from the host frame scheduler to submit its configured transaction. VALID value is also reported in case of ACK acknowledge is received on a double-buffered channel. In this case the channel remains active on the alternate buffer while application needs to read the current buffer and toggle DTOGTX. In case software is late in reading and the alternate buffer is not ready, the host channel is automatically suspended transparently to the application. The suspended double buffered channel is re-activated as soon as delay is recovered and DTOGTX is toggled. - NAK NAK value is reported in case of NAK acknowledge received. When in NAK state the channel is suspended and does not try to transmit. NAK state is moved to VALID by hardware at the start of the next frame, or software can change it to immediately retry transmission by writing it to VALID, or can disable it and abort the transaction by writing DISABLE - STALL STALL value is reported in case of STALL acknowledge received. When in STALL state the channel behaves as disabled. Application must not retry transmission but reset the USB and re-enumerate."]
pub type StatrxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTOGRX` writer - Data Toggle, for reception transfers If the endpoint/channel is not isochronous, this bit contains the expected value of the data toggle bit (01=1DATA0, 11=1DATA1) for the next data packet to be received. Hardware toggles this bit, when the ACK handshake is sent following a data packet reception having a matching data PID value; if the endpoint is defined as a control one, hardware clears this bit at the reception of a SETUP PID received from host (in device mode), while it sets this bit to 1 when SETUP transaction is acknowledged by device (in host mode). If the endpoint/channel is using the double-buffering feature this bit is used to support packet buffer swapping too (Refer to Section134.5.3: Double-buffered endpoints and usage in Device mode). If the endpoint/channel is isochronous, this bit is used only to support packet buffer swapping for data transmission since no data toggling is used for this kind of channels/endpoints and only DATA0 packet are transmitted (Refer to Section134.5.5: Isochronous transfers in Device mode). Hardware toggles this bit just after the end of data packet reception, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint is not a control one) or to force specific data toggle/packet buffer usage. When the application software writes 0, the value of DTOGRX remains unchanged, while writing 1 makes the bit value toggle. This bit is read/write but it can be only toggled by writing 1."]
pub type DtogrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRX` reader - USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the CTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STATRX field of this register. One NAKed transaction keeps pending and is automatically retried by the host at the next frame, or the host can immediately retry by resetting STATRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STATRX field of this register. Host application must consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STATRX field of this register. Host application must read received data from USBRAM and re-arm the channel by writing VALID to the STATRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STATRX field of this register. Host application must read received data from USBRAM and toggle the DTOGTX bit of this register. - A transaction ended with error sets this bit. Errors can be seen via the bits ERR_RX (host mode only). This bit is read/write but only 0 can be written, writing 1 has no effect."]
pub type VtrxR = crate::BitReader;
#[doc = "Field `VTRX` writer - USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the CTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STATRX field of this register. One NAKed transaction keeps pending and is automatically retried by the host at the next frame, or the host can immediately retry by resetting STATRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STATRX field of this register. Host application must consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STATRX field of this register. Host application must read received data from USBRAM and re-arm the channel by writing VALID to the STATRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STATRX field of this register. Host application must read received data from USBRAM and toggle the DTOGTX bit of this register. - A transaction ended with error sets this bit. Errors can be seen via the bits ERR_RX (host mode only). This bit is read/write but only 0 can be written, writing 1 has no effect."]
pub type VtrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVADDR` reader - Host mode Device address assigned to the endpoint during the enumeration process."]
pub type DevaddrR = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Host mode Device address assigned to the endpoint during the enumeration process."]
pub type DevaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NAK` reader - Host mode This bit is set by the hardware when a device responds with a NAK. Software can use this bit to monitor the number of NAKs received from a device."]
pub type NakR = crate::BitReader;
#[doc = "Field `NAK` writer - Host mode This bit is set by the hardware when a device responds with a NAK. Software can use this bit to monitor the number of NAKs received from a device."]
pub type NakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low speed endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LsEp {
    #[doc = "0: Full speed endpoint"]
    B0x0 = 0,
    #[doc = "1: Low speed endpoint"]
    B0x1 = 1,
}
impl From<LsEp> for bool {
    #[inline(always)]
    fn from(variant: LsEp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LS_EP` reader - Low speed endpoint"]
pub type LsEpR = crate::BitReader<LsEp>;
impl LsEpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LsEp {
        match self.bits {
            false => LsEp::B0x0,
            true => LsEp::B0x1,
        }
    }
    #[doc = "Full speed endpoint"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LsEp::B0x0
    }
    #[doc = "Low speed endpoint"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LsEp::B0x1
    }
}
#[doc = "Field `LS_EP` writer - Low speed endpoint"]
pub type LsEpW<'a, REG> = crate::BitWriter<'a, REG, LsEp>;
impl<'a, REG> LsEpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full speed endpoint"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LsEp::B0x0)
    }
    #[doc = "Low speed endpoint"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LsEp::B0x1)
    }
}
#[doc = "Field `ERR_TX` reader - Received error for an OUT/SETUP transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
pub type ErrTxR = crate::BitReader;
#[doc = "Field `ERR_TX` writer - Received error for an OUT/SETUP transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
pub type ErrTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_RX` reader - Received error for an IN transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
pub type ErrRxR = crate::BitReader;
#[doc = "Field `ERR_RX` writer - Received error for an IN transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
pub type ErrRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Three errors for an OUT or SETUP transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an OUT transaction. THREE_ERR_TX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ThreeErrTx {
    #[doc = "0: Less than 3 errors received."]
    B0x0 = 0,
    #[doc = "1: More than 3 errors received, last error is timeout error."]
    B0x1 = 1,
    #[doc = "2: More than 3 errors received, last error is data error (CRC error)."]
    B0x2 = 2,
    #[doc = "3: More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing error, SYNC error)."]
    B0x3 = 3,
}
impl From<ThreeErrTx> for u8 {
    #[inline(always)]
    fn from(variant: ThreeErrTx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ThreeErrTx {
    type Ux = u8;
}
impl crate::IsEnum for ThreeErrTx {}
#[doc = "Field `THREE_ERR_TX` reader - Three errors for an OUT or SETUP transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an OUT transaction. THREE_ERR_TX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
pub type ThreeErrTxR = crate::FieldReader<ThreeErrTx>;
impl ThreeErrTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ThreeErrTx {
        match self.bits {
            0 => ThreeErrTx::B0x0,
            1 => ThreeErrTx::B0x1,
            2 => ThreeErrTx::B0x2,
            3 => ThreeErrTx::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Less than 3 errors received."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ThreeErrTx::B0x0
    }
    #[doc = "More than 3 errors received, last error is timeout error."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ThreeErrTx::B0x1
    }
    #[doc = "More than 3 errors received, last error is data error (CRC error)."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ThreeErrTx::B0x2
    }
    #[doc = "More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing error, SYNC error)."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ThreeErrTx::B0x3
    }
}
#[doc = "Field `THREE_ERR_TX` writer - Three errors for an OUT or SETUP transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an OUT transaction. THREE_ERR_TX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
pub type ThreeErrTxW<'a, REG> = crate::FieldWriter<'a, REG, 2, ThreeErrTx, crate::Safe>;
impl<'a, REG> ThreeErrTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Less than 3 errors received."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ThreeErrTx::B0x0)
    }
    #[doc = "More than 3 errors received, last error is timeout error."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ThreeErrTx::B0x1)
    }
    #[doc = "More than 3 errors received, last error is data error (CRC error)."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ThreeErrTx::B0x2)
    }
    #[doc = "More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing error, SYNC error)."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ThreeErrTx::B0x3)
    }
}
#[doc = "Three errors for an IN transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an IN transaction. THREE_ERR_RX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ThreeErrRx {
    #[doc = "0: Less than 3 errors received."]
    B0x0 = 0,
    #[doc = "1: More than 3 errors received, last error is timeout error."]
    B0x1 = 1,
    #[doc = "2: More than 3 errors received, last error is data error (CRC error)."]
    B0x2 = 2,
    #[doc = "3: More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing error, SYNC error)."]
    B0x3 = 3,
}
impl From<ThreeErrRx> for u8 {
    #[inline(always)]
    fn from(variant: ThreeErrRx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ThreeErrRx {
    type Ux = u8;
}
impl crate::IsEnum for ThreeErrRx {}
#[doc = "Field `THREE_ERR_RX` reader - Three errors for an IN transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an IN transaction. THREE_ERR_RX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
pub type ThreeErrRxR = crate::FieldReader<ThreeErrRx>;
impl ThreeErrRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ThreeErrRx {
        match self.bits {
            0 => ThreeErrRx::B0x0,
            1 => ThreeErrRx::B0x1,
            2 => ThreeErrRx::B0x2,
            3 => ThreeErrRx::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Less than 3 errors received."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ThreeErrRx::B0x0
    }
    #[doc = "More than 3 errors received, last error is timeout error."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ThreeErrRx::B0x1
    }
    #[doc = "More than 3 errors received, last error is data error (CRC error)."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ThreeErrRx::B0x2
    }
    #[doc = "More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing error, SYNC error)."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ThreeErrRx::B0x3
    }
}
#[doc = "Field `THREE_ERR_RX` writer - Three errors for an IN transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an IN transaction. THREE_ERR_RX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
pub type ThreeErrRxW<'a, REG> = crate::FieldWriter<'a, REG, 2, ThreeErrRx, crate::Safe>;
impl<'a, REG> ThreeErrRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Less than 3 errors received."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ThreeErrRx::B0x0)
    }
    #[doc = "More than 3 errors received, last error is timeout error."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ThreeErrRx::B0x1)
    }
    #[doc = "More than 3 errors received, last error is data error (CRC error)."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ThreeErrRx::B0x2)
    }
    #[doc = "More than 3 errors received, last error is protocol error (invalid PID, false EOP, bitstuffing error, SYNC error)."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ThreeErrRx::B0x3)
    }
}
impl R {
    #[doc = "Bits 0:3 - endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction."]
    #[inline(always)]
    pub fn ea(&self) -> EaR {
        EaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written. Host mode Same as VTRX behavior but for USB OUT and SETUP transactions."]
    #[inline(always)]
    pub fn vttx(&self) -> VttxR {
        VttxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the UTYPE bits. Table1217 summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Section134.5.3: Double-buffered endpoints and usage in Device mode. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered STALL instead of ACK. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required."]
    #[inline(always)]
    pub fn epkind(&self) -> EpkindR {
        EpkindR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - USB type of transaction These bits configure the behavior of this endpoint/channel as described in Table1216: Endpoint/channel type encoding. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral does not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet is accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of isochronous channels/endpoints is explained in Section134.5.5: Isochronous transfers in Device mode"]
    #[inline(always)]
    pub fn utype(&self) -> UtypeR {
        UtypeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed Device mode This bit is read-only and it is set by the hardware when the last completed transaction is a SETUP. This bit changes its value only for control endpoints. It must be examined, in the case of a successful receive transaction (VTRX event), to determine the type of transaction occurred. To protect the interrupt service routine from the changes in SETUP bits due to next incoming tokens, this bit is kept frozen while VTRX bit is at 1; its state changes when VTRX is at 0. This bit is read-only. Host mode This bit is set by the software to send a SETUP transaction on a control endpoint. This bit changes its value only for control endpoints. It is cleared by hardware when the SETUP transaction is acknowledged and VTTX interrupt generated."]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the CTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STATRX field of this register. One NAKed transaction keeps pending and is automatically retried by the host at the next frame, or the host can immediately retry by resetting STATRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STATRX field of this register. Host application must consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STATRX field of this register. Host application must read received data from USBRAM and re-arm the channel by writing VALID to the STATRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STATRX field of this register. Host application must read received data from USBRAM and toggle the DTOGTX bit of this register. - A transaction ended with error sets this bit. Errors can be seen via the bits ERR_RX (host mode only). This bit is read/write but only 0 can be written, writing 1 has no effect."]
    #[inline(always)]
    pub fn vtrx(&self) -> VtrxR {
        VtrxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Host mode Device address assigned to the endpoint during the enumeration process."]
    #[inline(always)]
    pub fn devaddr(&self) -> DevaddrR {
        DevaddrR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Host mode This bit is set by the hardware when a device responds with a NAK. Software can use this bit to monitor the number of NAKs received from a device."]
    #[inline(always)]
    pub fn nak(&self) -> NakR {
        NakR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Low speed endpoint"]
    #[inline(always)]
    pub fn ls_ep(&self) -> LsEpR {
        LsEpR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Received error for an OUT/SETUP transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
    #[inline(always)]
    pub fn err_tx(&self) -> ErrTxR {
        ErrTxR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Received error for an IN transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
    #[inline(always)]
    pub fn err_rx(&self) -> ErrRxR {
        ErrRxR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Three errors for an OUT or SETUP transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an OUT transaction. THREE_ERR_TX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
    #[inline(always)]
    pub fn three_err_tx(&self) -> ThreeErrTxR {
        ThreeErrTxR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - Three errors for an IN transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an IN transaction. THREE_ERR_RX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
    #[inline(always)]
    pub fn three_err_rx(&self) -> ThreeErrRxR {
        ThreeErrRxR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - endpoint/channel address Device mode Software must write in this field the 4-bit address used to identify the transactions directed to this endpoint. A value must be written before enabling the corresponding endpoint. Host mode Software must write in this field the 4-bit address used to identify the channel addressed by the host transaction."]
    #[inline(always)]
    #[must_use]
    pub fn ea(&mut self) -> EaW<UsbChep1rSpec> {
        EaW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    #[must_use]
    pub fn stattx(&mut self) -> StattxW<UsbChep1rSpec> {
        StattxW::new(self, 4)
    }
    #[doc = "Bit 6 - Data toggle, for transmission transfers If the endpoint/channel is non-isochronous, this bit contains the required value of the data toggle bit (01=1DATA0, 11=1DATA1) for the next data packet to be transmitted. Hardware toggles this bit when the ACK handshake is received from the USB host, following a data packet transmission. If the endpoint/channel is defined as a control one, hardware sets this bit to 1 at the reception of a SETUP PID addressed to this endpoint (in device mode) or when a SETUP transaction is acknowledged by the device (in host mode). If the endpoint/channel is using the double buffer feature, this bit is used to support packet buffer swapping too (Refer to Section134.5.3: Double-buffered endpoints and usage in Device mode). If the endpoint/channel is isochronous, this bit is used to support packet buffer swapping since no data toggling is used for this sort of endpoints and only DATA0 packet are transmitted (refer to Section134.5.5: Isochronous transfers in Device mode). Hardware toggles this bit just after the end of data packet transmission, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint/channel is not a control one) or to force a specific data toggle/packet buffer usage. When the application software writes 0, the value of DTOGTX remains unchanged, while writing 1 makes the bit value to toggle. This bit is read/write but it can only be toggled by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn dtogtx(&mut self) -> DtogtxW<UsbChep1rSpec> {
        DtogtxW::new(self, 6)
    }
    #[doc = "Bit 7 - Valid USB transaction transmitted Device mode This bit is set by the hardware when an IN transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in the USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written. Host mode Same as VTRX behavior but for USB OUT and SETUP transactions."]
    #[inline(always)]
    #[must_use]
    pub fn vttx(&mut self) -> VttxW<UsbChep1rSpec> {
        VttxW::new(self, 7)
    }
    #[doc = "Bit 8 - endpoint/channel kind The meaning of this bit depends on the endpoint/channel type configured by the UTYPE bits. Table1217 summarizes the different meanings. DBL_BUF: This bit is set by the software to enable the double-buffering feature for this bulk endpoint. The usage of double-buffered bulk endpoints is explained in Section134.5.3: Double-buffered endpoints and usage in Device mode. STATUS_OUT: This bit is set by the software to indicate that a status out transaction is expected: in this case all OUT transactions containing more than zero data bytes are answered STALL instead of ACK. This bit may be used to improve the robustness of the application to protocol errors during control transfers and its usage is intended for control endpoints only. When STATUS_OUT is reset, OUT transactions can have any number of bytes, as required."]
    #[inline(always)]
    #[must_use]
    pub fn epkind(&mut self) -> EpkindW<UsbChep1rSpec> {
        EpkindW::new(self, 8)
    }
    #[doc = "Bits 9:10 - USB type of transaction These bits configure the behavior of this endpoint/channel as described in Table1216: Endpoint/channel type encoding. Channel0/Endpoint0 must always be a control endpoint/channel and each USB function must have at least one control endpoint/channel which has address 0, but there may be other control channels/endpoints if required. Only control channels/endpoints handle SETUP transactions, which are ignored by endpoints of other kinds. SETUP transactions cannot be answered with NAK or STALL. If a control endpoint/channel is defined as NAK, the USB peripheral does not answer, simulating a receive error, in the receive direction when a SETUP transaction is received. If the control endpoint/channel is defined as STALL in the receive direction, then the SETUP packet is accepted anyway, transferring data and issuing the CTR interrupt. The reception of OUT transactions is handled in the normal way, even if the endpoint/channel is a control one. Bulk and interrupt endpoints have very similar behavior and they differ only in the special feature available using the EPKIND configuration bit. The usage of isochronous channels/endpoints is explained in Section134.5.5: Isochronous transfers in Device mode"]
    #[inline(always)]
    #[must_use]
    pub fn utype(&mut self) -> UtypeW<UsbChep1rSpec> {
        UtypeW::new(self, 9)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers Device mode These bits contain information about the endpoint status, which are listed in Table1215: Reception status encoding on page11025. These bits can be toggled by software to initialize their value. When the application software writes 0, the value remains unchanged, while writing 1 makes the bit value to toggle. Hardware sets the STATRX bits to NAK when a correct transfer has occurred (VTRX1=11) corresponding to a OUT or SETUP (control only) transaction addressed to this endpoint, so the software has the time to elaborate the received data before it acknowledges a new transaction. Double-buffered bulk endpoints implement a special transaction flow control, which control the status based upon buffer availability condition (Refer to Section134.5.3: Double-buffered endpoints and usage in Device mode). If the endpoint is defined as isochronous, its status can be only VALID or DISABLED, so that the hardware cannot change the status of the endpoint after a successful transaction. If the software sets the STATRX bits to STALL or NAK for an isochronous endpoint, the USB peripheral behavior is not defined. These bits are read/write but they can be only toggled by writing 1. Host mode These bits are the host application controls to start, retry, or abort host transactions driven by the channel. These bits also contain information about the device answer to the last IN channel transaction and report the current status of the channel according to the following STATRX table of states: - DISABLE DISABLE value is reported in case of ACK acknowledge is received on a single-buffer channel. When in DISABLE state the channel is unused or not active waiting for application to restart it by writing VALID. Application can reset a VALID channel to DISABLE to abort a transaction. In this case the transaction is immediately removed from the host execution list. If the aborted transaction was already under execution it is regularly terminated on the USB but the relative VTRX interrupt is not generated. - VALID A host channel is actively trying to submit USB transaction to device only when in VALID state.VALID state can be set by software or automatically by hardware on a NAKED channel at the start of a new frame. When set to VALID, an host channel enters the host execution queue and waits permission from the host frame scheduler to submit its configured transaction. VALID value is also reported in case of ACK acknowledge is received on a double-buffered channel. In this case the channel remains active on the alternate buffer while application needs to read the current buffer and toggle DTOGTX. In case software is late in reading and the alternate buffer is not ready, the host channel is automatically suspended transparently to the application. The suspended double buffered channel is re-activated as soon as delay is recovered and DTOGTX is toggled. - NAK NAK value is reported in case of NAK acknowledge received. When in NAK state the channel is suspended and does not try to transmit. NAK state is moved to VALID by hardware at the start of the next frame, or software can change it to immediately retry transmission by writing it to VALID, or can disable it and abort the transaction by writing DISABLE - STALL STALL value is reported in case of STALL acknowledge received. When in STALL state the channel behaves as disabled. Application must not retry transmission but reset the USB and re-enumerate."]
    #[inline(always)]
    #[must_use]
    pub fn statrx(&mut self) -> StatrxW<UsbChep1rSpec> {
        StatrxW::new(self, 12)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers If the endpoint/channel is not isochronous, this bit contains the expected value of the data toggle bit (01=1DATA0, 11=1DATA1) for the next data packet to be received. Hardware toggles this bit, when the ACK handshake is sent following a data packet reception having a matching data PID value; if the endpoint is defined as a control one, hardware clears this bit at the reception of a SETUP PID received from host (in device mode), while it sets this bit to 1 when SETUP transaction is acknowledged by device (in host mode). If the endpoint/channel is using the double-buffering feature this bit is used to support packet buffer swapping too (Refer to Section134.5.3: Double-buffered endpoints and usage in Device mode). If the endpoint/channel is isochronous, this bit is used only to support packet buffer swapping for data transmission since no data toggling is used for this kind of channels/endpoints and only DATA0 packet are transmitted (Refer to Section134.5.5: Isochronous transfers in Device mode). Hardware toggles this bit just after the end of data packet reception, since no handshake is used for isochronous transfers. This bit can also be toggled by the software to initialize its value (mandatory when the endpoint is not a control one) or to force specific data toggle/packet buffer usage. When the application software writes 0, the value of DTOGRX remains unchanged, while writing 1 makes the bit value toggle. This bit is read/write but it can be only toggled by writing 1."]
    #[inline(always)]
    #[must_use]
    pub fn dtogrx(&mut self) -> DtogrxW<UsbChep1rSpec> {
        DtogrxW::new(self, 14)
    }
    #[doc = "Bit 15 - USB valid transaction received Device mode This bit is set by the hardware when an OUT/SETUP transaction is successfully completed on this endpoint; the software can only clear this bit. If the CTRM bit in USB_CNTR register is set accordingly, a generic interrupt condition is generated together with the endpoint related interrupt condition, which is always activated. The type of occurred transaction, OUT or SETUP, can be determined from the SETUP bit described below. A transaction ended with a NAK or STALL handshake does not set this bit, since no data is actually transferred, as in the case of protocol errors or data toggle mismatches. This bit is read/write but only 0 can be written, writing 1 has no effect. Host mode This bit is set by the hardware when an IN transaction is successfully completed on this channel. The software can only clear this bit. If the CTRM bit in USB_CNTR register is set a generic interrupt condition is generated together with the channel related flag, which is always activated. - A transaction ended with a NAK sets this bit and NAK answer is reported to application reading the NAK state from the STATRX field of this register. One NAKed transaction keeps pending and is automatically retried by the host at the next frame, or the host can immediately retry by resetting STATRX state to VALID. - A transaction ended by STALL handshake sets this bit and the STALL answer is reported to application reading the STALL state from the STATRX field of this register. Host application must consequently disable the channel and re-enumerate. - A transaction ended with ACK handshake sets this bit If double buffering is disabled, ACK answer is reported by application reading the DISABLE state from the STATRX field of this register. Host application must read received data from USBRAM and re-arm the channel by writing VALID to the STATRX field of this register. If double buffering is enabled, ACK answer is reported by application reading VALID state from the STATRX field of this register. Host application must read received data from USBRAM and toggle the DTOGTX bit of this register. - A transaction ended with error sets this bit. Errors can be seen via the bits ERR_RX (host mode only). This bit is read/write but only 0 can be written, writing 1 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn vtrx(&mut self) -> VtrxW<UsbChep1rSpec> {
        VtrxW::new(self, 15)
    }
    #[doc = "Bits 16:22 - Host mode Device address assigned to the endpoint during the enumeration process."]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DevaddrW<UsbChep1rSpec> {
        DevaddrW::new(self, 16)
    }
    #[doc = "Bit 23 - Host mode This bit is set by the hardware when a device responds with a NAK. Software can use this bit to monitor the number of NAKs received from a device."]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NakW<UsbChep1rSpec> {
        NakW::new(self, 23)
    }
    #[doc = "Bit 24 - Low speed endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn ls_ep(&mut self) -> LsEpW<UsbChep1rSpec> {
        LsEpW::new(self, 24)
    }
    #[doc = "Bit 25 - Received error for an OUT/SETUP transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an OUT or SETUP transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
    #[inline(always)]
    #[must_use]
    pub fn err_tx(&mut self) -> ErrTxW<UsbChep1rSpec> {
        ErrTxW::new(self, 25)
    }
    #[doc = "Bit 26 - Received error for an IN transaction Host mode This bit is set by the hardware when an error (for example no answer by the device, CRC error, bit stuffing error, framing format violation, etc.) has occurred during an IN transaction on this channel. The software can only clear this bit. If the ERRM bit in USB_CNTR register is set, a generic interrupt condition is generated together with the channel related flag, which is always activated."]
    #[inline(always)]
    #[must_use]
    pub fn err_rx(&mut self) -> ErrRxW<UsbChep1rSpec> {
        ErrRxW::new(self, 26)
    }
    #[doc = "Bits 27:28 - Three errors for an OUT or SETUP transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an OUT transaction. THREE_ERR_TX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
    #[inline(always)]
    #[must_use]
    pub fn three_err_tx(&mut self) -> ThreeErrTxW<UsbChep1rSpec> {
        ThreeErrTxW::new(self, 27)
    }
    #[doc = "Bits 29:30 - Three errors for an IN transaction Host mode This bit is set by the hardware when 3 consecutive transaction errors occurred on the USB bus for an IN transaction. THREE_ERR_RX is not generated for isochronous transactions. The software can only clear this bit. Coding of the received error:"]
    #[inline(always)]
    #[must_use]
    pub fn three_err_rx(&mut self) -> ThreeErrRxW<UsbChep1rSpec> {
        ThreeErrRxW::new(self, 29)
    }
}
#[doc = "USB endpoint/channel 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbChep1rSpec;
impl crate::RegisterSpec for UsbChep1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_chep1r::R`](R) reader structure"]
impl crate::Readable for UsbChep1rSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_chep1r::W`](W) writer structure"]
impl crate::Writable for UsbChep1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB_CHEP1R to value 0"]
impl crate::Resettable for UsbChep1rSpec {
    const RESET_VALUE: u32 = 0;
}
