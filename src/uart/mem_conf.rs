#[doc = "Reader of register MEM_CONF"]
pub type R = crate::R<u32, super::MEM_CONF>;
#[doc = "Writer for register MEM_CONF"]
pub type W = crate::W<u32, super::MEM_CONF>;
#[doc = "Register MEM_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_MEM_EMPTY_THRHD`"]
pub type TX_MEM_EMPTY_THRHD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_MEM_EMPTY_THRHD`"]
pub struct TX_MEM_EMPTY_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MEM_EMPTY_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `RX_MEM_FULL_THRHD`"]
pub type RX_MEM_FULL_THRHD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_MEM_FULL_THRHD`"]
pub struct RX_MEM_FULL_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MEM_FULL_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Reader of field `XOFF_THRESHOLD_H2`"]
pub type XOFF_THRESHOLD_H2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOFF_THRESHOLD_H2`"]
pub struct XOFF_THRESHOLD_H2_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFF_THRESHOLD_H2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
#[doc = "Reader of field `XON_THRESHOLD_H2`"]
pub type XON_THRESHOLD_H2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XON_THRESHOLD_H2`"]
pub struct XON_THRESHOLD_H2_W<'a> {
    w: &'a mut W,
}
impl<'a> XON_THRESHOLD_H2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `RX_TOUT_THRHD_H3`"]
pub type RX_TOUT_THRHD_H3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_TOUT_THRHD_H3`"]
pub struct RX_TOUT_THRHD_H3_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TOUT_THRHD_H3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `RX_FLOW_THRHD_H3`"]
pub type RX_FLOW_THRHD_H3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_FLOW_THRHD_H3`"]
pub struct RX_FLOW_THRHD_H3_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLOW_THRHD_H3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Reader of field `TX_SIZE`"]
pub type TX_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_SIZE`"]
pub struct TX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `RX_SIZE`"]
pub type RX_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_SIZE`"]
pub struct RX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Reader of field `MEM_PD`"]
pub type MEM_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_PD`"]
pub struct MEM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30 - refer to txfifo_empty_thrhd 's describtion."]
    #[inline(always)]
    pub fn tx_mem_empty_thrhd(&self) -> TX_MEM_EMPTY_THRHD_R {
        TX_MEM_EMPTY_THRHD_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 25:27 - refer to the rxfifo_full_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_mem_full_thrhd(&self) -> RX_MEM_FULL_THRHD_R {
        RX_MEM_FULL_THRHD_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 23:24 - refer to the uart_xoff_threshold's describtion."]
    #[inline(always)]
    pub fn xoff_threshold_h2(&self) -> XOFF_THRESHOLD_H2_R {
        XOFF_THRESHOLD_H2_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 21:22 - refer to the uart_xon_threshold's describtion."]
    #[inline(always)]
    pub fn xon_threshold_h2(&self) -> XON_THRESHOLD_H2_R {
        XON_THRESHOLD_H2_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - refer to the rx_tout_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_tout_thrhd_h3(&self) -> RX_TOUT_THRHD_H3_R {
        RX_TOUT_THRHD_H3_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - refer to the rx_flow_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_flow_thrhd_h3(&self) -> RX_FLOW_THRHD_H3_R {
        RX_FLOW_THRHD_H3_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 7:10 - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
    #[inline(always)]
    pub fn tx_size(&self) -> TX_SIZE_R {
        TX_SIZE_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 3:6 - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
    #[inline(always)]
    pub fn rx_size(&self) -> RX_SIZE_R {
        RX_SIZE_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
    #[inline(always)]
    pub fn mem_pd(&self) -> MEM_PD_R {
        MEM_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:30 - refer to txfifo_empty_thrhd 's describtion."]
    #[inline(always)]
    pub fn tx_mem_empty_thrhd(&mut self) -> TX_MEM_EMPTY_THRHD_W {
        TX_MEM_EMPTY_THRHD_W { w: self }
    }
    #[doc = "Bits 25:27 - refer to the rxfifo_full_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_mem_full_thrhd(&mut self) -> RX_MEM_FULL_THRHD_W {
        RX_MEM_FULL_THRHD_W { w: self }
    }
    #[doc = "Bits 23:24 - refer to the uart_xoff_threshold's describtion."]
    #[inline(always)]
    pub fn xoff_threshold_h2(&mut self) -> XOFF_THRESHOLD_H2_W {
        XOFF_THRESHOLD_H2_W { w: self }
    }
    #[doc = "Bits 21:22 - refer to the uart_xon_threshold's describtion."]
    #[inline(always)]
    pub fn xon_threshold_h2(&mut self) -> XON_THRESHOLD_H2_W {
        XON_THRESHOLD_H2_W { w: self }
    }
    #[doc = "Bits 18:20 - refer to the rx_tout_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_tout_thrhd_h3(&mut self) -> RX_TOUT_THRHD_H3_W {
        RX_TOUT_THRHD_H3_W { w: self }
    }
    #[doc = "Bits 15:17 - refer to the rx_flow_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_flow_thrhd_h3(&mut self) -> RX_FLOW_THRHD_H3_W {
        RX_FLOW_THRHD_H3_W { w: self }
    }
    #[doc = "Bits 7:10 - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
    #[inline(always)]
    pub fn tx_size(&mut self) -> TX_SIZE_W {
        TX_SIZE_W { w: self }
    }
    #[doc = "Bits 3:6 - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
    #[inline(always)]
    pub fn rx_size(&mut self) -> RX_SIZE_W {
        RX_SIZE_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
    #[inline(always)]
    pub fn mem_pd(&mut self) -> MEM_PD_W {
        MEM_PD_W { w: self }
    }
}
