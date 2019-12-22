#[doc = "Reader of register RMT_CH1STATUS_REG"]
pub type R = crate::R<u32, super::RMT_CH1STATUS_REG>;
#[doc = "Writer for register RMT_CH1STATUS_REG"]
pub type W = crate::W<u32, super::RMT_CH1STATUS_REG>;
#[doc = "Register RMT_CH1STATUS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_CH1STATUS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_STATUS_CH1`"]
pub type RMT_STATUS_CH1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RMT_STATUS_CH1`"]
pub struct RMT_STATUS_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_STATUS_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
#[doc = "Reader of field `RMT_APB_MEM_RD_ERR_CH1`"]
pub type RMT_APB_MEM_RD_ERR_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_APB_MEM_RD_ERR_CH1`"]
pub struct RMT_APB_MEM_RD_ERR_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_APB_MEM_RD_ERR_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RMT_APB_MEM_WR_ERR_CH1`"]
pub type RMT_APB_MEM_WR_ERR_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_APB_MEM_WR_ERR_CH1`"]
pub struct RMT_APB_MEM_WR_ERR_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_APB_MEM_WR_ERR_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RMT_MEM_EMPTY_CH1`"]
pub type RMT_MEM_EMPTY_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_EMPTY_CH1`"]
pub struct RMT_MEM_EMPTY_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_EMPTY_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `RMT_MEM_FULL_CH1`"]
pub type RMT_MEM_FULL_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_FULL_CH1`"]
pub struct RMT_MEM_FULL_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_FULL_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `RMT_MEM_OWNER_ERR_CH1`"]
pub type RMT_MEM_OWNER_ERR_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_OWNER_ERR_CH1`"]
pub struct RMT_MEM_OWNER_ERR_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_OWNER_ERR_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `RMT_STATE_CH1`"]
pub type RMT_STATE_CH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_STATE_CH1`"]
pub struct RMT_STATE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_STATE_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `RMT_MEM_RADDR_EX_CH1`"]
pub type RMT_MEM_RADDR_EX_CH1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RMT_MEM_RADDR_EX_CH1`"]
pub struct RMT_MEM_RADDR_EX_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_RADDR_EX_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 12)) | (((value as u32) & 0x03ff) << 12);
        self.w
    }
}
#[doc = "Reader of field `RMT_MEM_WADDR_EX_CH1`"]
pub type RMT_MEM_WADDR_EX_CH1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RMT_MEM_WADDR_EX_CH1`"]
pub struct RMT_MEM_WADDR_EX_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_WADDR_EX_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The status for channel1"]
    #[inline(always)]
    pub fn rmt_status_ch1(&self) -> RMT_STATUS_CH1_R {
        RMT_STATUS_CH1_R::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bit 31 - The apb read memory status bit for channel1 turns to high level when the apb read address exceeds the configuration range."]
    #[inline(always)]
    pub fn rmt_apb_mem_rd_err_ch1(&self) -> RMT_APB_MEM_RD_ERR_CH1_R {
        RMT_APB_MEM_RD_ERR_CH1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - The apb write memory status bit for channel1 turns to high level when the apb write address exceeds the configuration range."]
    #[inline(always)]
    pub fn rmt_apb_mem_wr_err_ch1(&self) -> RMT_APB_MEM_WR_ERR_CH1_R {
        RMT_APB_MEM_WR_ERR_CH1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - The memory empty status bit for channel1. in acyclic mode, this bit turns to high level when mem_raddr_ex is greater than or equal to the configured range."]
    #[inline(always)]
    pub fn rmt_mem_empty_ch1(&self) -> RMT_MEM_EMPTY_CH1_R {
        RMT_MEM_EMPTY_CH1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - The memory full status bit for channel1 turns to high level when mem_waddr_ex is greater than or equal to the configuration range."]
    #[inline(always)]
    pub fn rmt_mem_full_ch1(&self) -> RMT_MEM_FULL_CH1_R {
        RMT_MEM_FULL_CH1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - When channel1 is configured for receive mode, this bit will turn to high level if rmt_mem_owner register is not set to 1."]
    #[inline(always)]
    pub fn rmt_mem_owner_err_ch1(&self) -> RMT_MEM_OWNER_ERR_CH1_R {
        RMT_MEM_OWNER_ERR_CH1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - The channel1 state machine status register.3'h0 : idle, 3'h1 : send, 3'h2 : read memory, 3'h3 : receive, 3'h4 : wait."]
    #[inline(always)]
    pub fn rmt_state_ch1(&self) -> RMT_STATE_CH1_R {
        RMT_STATE_CH1_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 12:21 - The current memory write address of channel1."]
    #[inline(always)]
    pub fn rmt_mem_raddr_ex_ch1(&self) -> RMT_MEM_RADDR_EX_CH1_R {
        RMT_MEM_RADDR_EX_CH1_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - The current memory read address of channel1."]
    #[inline(always)]
    pub fn rmt_mem_waddr_ex_ch1(&self) -> RMT_MEM_WADDR_EX_CH1_R {
        RMT_MEM_WADDR_EX_CH1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:31 - The status for channel1"]
    #[inline(always)]
    pub fn rmt_status_ch1(&mut self) -> RMT_STATUS_CH1_W {
        RMT_STATUS_CH1_W { w: self }
    }
    #[doc = "Bit 31 - The apb read memory status bit for channel1 turns to high level when the apb read address exceeds the configuration range."]
    #[inline(always)]
    pub fn rmt_apb_mem_rd_err_ch1(&mut self) -> RMT_APB_MEM_RD_ERR_CH1_W {
        RMT_APB_MEM_RD_ERR_CH1_W { w: self }
    }
    #[doc = "Bit 30 - The apb write memory status bit for channel1 turns to high level when the apb write address exceeds the configuration range."]
    #[inline(always)]
    pub fn rmt_apb_mem_wr_err_ch1(&mut self) -> RMT_APB_MEM_WR_ERR_CH1_W {
        RMT_APB_MEM_WR_ERR_CH1_W { w: self }
    }
    #[doc = "Bit 29 - The memory empty status bit for channel1. in acyclic mode, this bit turns to high level when mem_raddr_ex is greater than or equal to the configured range."]
    #[inline(always)]
    pub fn rmt_mem_empty_ch1(&mut self) -> RMT_MEM_EMPTY_CH1_W {
        RMT_MEM_EMPTY_CH1_W { w: self }
    }
    #[doc = "Bit 28 - The memory full status bit for channel1 turns to high level when mem_waddr_ex is greater than or equal to the configuration range."]
    #[inline(always)]
    pub fn rmt_mem_full_ch1(&mut self) -> RMT_MEM_FULL_CH1_W {
        RMT_MEM_FULL_CH1_W { w: self }
    }
    #[doc = "Bit 27 - When channel1 is configured for receive mode, this bit will turn to high level if rmt_mem_owner register is not set to 1."]
    #[inline(always)]
    pub fn rmt_mem_owner_err_ch1(&mut self) -> RMT_MEM_OWNER_ERR_CH1_W {
        RMT_MEM_OWNER_ERR_CH1_W { w: self }
    }
    #[doc = "Bits 24:26 - The channel1 state machine status register.3'h0 : idle, 3'h1 : send, 3'h2 : read memory, 3'h3 : receive, 3'h4 : wait."]
    #[inline(always)]
    pub fn rmt_state_ch1(&mut self) -> RMT_STATE_CH1_W {
        RMT_STATE_CH1_W { w: self }
    }
    #[doc = "Bits 12:21 - The current memory write address of channel1."]
    #[inline(always)]
    pub fn rmt_mem_raddr_ex_ch1(&mut self) -> RMT_MEM_RADDR_EX_CH1_W {
        RMT_MEM_RADDR_EX_CH1_W { w: self }
    }
    #[doc = "Bits 0:9 - The current memory read address of channel1."]
    #[inline(always)]
    pub fn rmt_mem_waddr_ex_ch1(&mut self) -> RMT_MEM_WADDR_EX_CH1_W {
        RMT_MEM_WADDR_EX_CH1_W { w: self }
    }
}
