#[doc = "Reader of register CH4STATUS"]
pub type R = crate::R<u32, super::CH4STATUS>;
#[doc = "Writer for register CH4STATUS"]
pub type W = crate::W<u32, super::CH4STATUS>;
#[doc = "Register CH4STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::CH4STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATUS_CH4`"]
pub type STATUS_CH4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STATUS_CH4`"]
pub struct STATUS_CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_CH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
#[doc = "Reader of field `APB_MEM_RD_ERR_CH4`"]
pub type APB_MEM_RD_ERR_CH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_MEM_RD_ERR_CH4`"]
pub struct APB_MEM_RD_ERR_CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_MEM_RD_ERR_CH4_W<'a> {
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
#[doc = "Reader of field `APB_MEM_WR_ERR_CH4`"]
pub type APB_MEM_WR_ERR_CH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_MEM_WR_ERR_CH4`"]
pub struct APB_MEM_WR_ERR_CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_MEM_WR_ERR_CH4_W<'a> {
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
#[doc = "Reader of field `MEM_EMPTY_CH4`"]
pub type MEM_EMPTY_CH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_EMPTY_CH4`"]
pub struct MEM_EMPTY_CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_EMPTY_CH4_W<'a> {
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
#[doc = "Reader of field `MEM_FULL_CH4`"]
pub type MEM_FULL_CH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_FULL_CH4`"]
pub struct MEM_FULL_CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FULL_CH4_W<'a> {
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
#[doc = "Reader of field `MEM_OWNER_ERR_CH4`"]
pub type MEM_OWNER_ERR_CH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_OWNER_ERR_CH4`"]
pub struct MEM_OWNER_ERR_CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_OWNER_ERR_CH4_W<'a> {
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
#[doc = "Reader of field `STATE_CH4`"]
pub type STATE_CH4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATE_CH4`"]
pub struct STATE_CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_CH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `MEM_RADDR_EX_CH4`"]
pub type MEM_RADDR_EX_CH4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_RADDR_EX_CH4`"]
pub struct MEM_RADDR_EX_CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_RADDR_EX_CH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 12)) | (((value as u32) & 0x03ff) << 12);
        self.w
    }
}
#[doc = "Reader of field `MEM_WADDR_EX_CH4`"]
pub type MEM_WADDR_EX_CH4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_WADDR_EX_CH4`"]
pub struct MEM_WADDR_EX_CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_WADDR_EX_CH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The status for channel4"]
    #[inline(always)]
    pub fn status_ch4(&self) -> STATUS_CH4_R {
        STATUS_CH4_R::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bit 31 - The apb read memory status bit for channel4 turns to high level when the apb read address exceeds the configuration range."]
    #[inline(always)]
    pub fn apb_mem_rd_err_ch4(&self) -> APB_MEM_RD_ERR_CH4_R {
        APB_MEM_RD_ERR_CH4_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - The apb write memory status bit for channel4 turns to high level when the apb write address exceeds the configuration range."]
    #[inline(always)]
    pub fn apb_mem_wr_err_ch4(&self) -> APB_MEM_WR_ERR_CH4_R {
        APB_MEM_WR_ERR_CH4_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - The memory empty status bit for channel4. in acyclic mode, this bit turns to high level when mem_raddr_ex is greater than or equal to the configured range."]
    #[inline(always)]
    pub fn mem_empty_ch4(&self) -> MEM_EMPTY_CH4_R {
        MEM_EMPTY_CH4_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - The memory full status bit for channel4 turns to high level when mem_waddr_ex is greater than or equal to the configuration range."]
    #[inline(always)]
    pub fn mem_full_ch4(&self) -> MEM_FULL_CH4_R {
        MEM_FULL_CH4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - When channel4 is configured for receive mode, this bit will turn to high level if rmt_mem_owner register is not set to 1."]
    #[inline(always)]
    pub fn mem_owner_err_ch4(&self) -> MEM_OWNER_ERR_CH4_R {
        MEM_OWNER_ERR_CH4_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - The channel4 state machine status register.3'h0 : idle, 3'h1 : send, 3'h2 : read memory, 3'h3 : receive, 3'h4 : wait."]
    #[inline(always)]
    pub fn state_ch4(&self) -> STATE_CH4_R {
        STATE_CH4_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 12:21 - The current memory write address of channel4."]
    #[inline(always)]
    pub fn mem_raddr_ex_ch4(&self) -> MEM_RADDR_EX_CH4_R {
        MEM_RADDR_EX_CH4_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - The current memory read address of channel4."]
    #[inline(always)]
    pub fn mem_waddr_ex_ch4(&self) -> MEM_WADDR_EX_CH4_R {
        MEM_WADDR_EX_CH4_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:31 - The status for channel4"]
    #[inline(always)]
    pub fn status_ch4(&mut self) -> STATUS_CH4_W {
        STATUS_CH4_W { w: self }
    }
    #[doc = "Bit 31 - The apb read memory status bit for channel4 turns to high level when the apb read address exceeds the configuration range."]
    #[inline(always)]
    pub fn apb_mem_rd_err_ch4(&mut self) -> APB_MEM_RD_ERR_CH4_W {
        APB_MEM_RD_ERR_CH4_W { w: self }
    }
    #[doc = "Bit 30 - The apb write memory status bit for channel4 turns to high level when the apb write address exceeds the configuration range."]
    #[inline(always)]
    pub fn apb_mem_wr_err_ch4(&mut self) -> APB_MEM_WR_ERR_CH4_W {
        APB_MEM_WR_ERR_CH4_W { w: self }
    }
    #[doc = "Bit 29 - The memory empty status bit for channel4. in acyclic mode, this bit turns to high level when mem_raddr_ex is greater than or equal to the configured range."]
    #[inline(always)]
    pub fn mem_empty_ch4(&mut self) -> MEM_EMPTY_CH4_W {
        MEM_EMPTY_CH4_W { w: self }
    }
    #[doc = "Bit 28 - The memory full status bit for channel4 turns to high level when mem_waddr_ex is greater than or equal to the configuration range."]
    #[inline(always)]
    pub fn mem_full_ch4(&mut self) -> MEM_FULL_CH4_W {
        MEM_FULL_CH4_W { w: self }
    }
    #[doc = "Bit 27 - When channel4 is configured for receive mode, this bit will turn to high level if rmt_mem_owner register is not set to 1."]
    #[inline(always)]
    pub fn mem_owner_err_ch4(&mut self) -> MEM_OWNER_ERR_CH4_W {
        MEM_OWNER_ERR_CH4_W { w: self }
    }
    #[doc = "Bits 24:26 - The channel4 state machine status register.3'h0 : idle, 3'h1 : send, 3'h2 : read memory, 3'h3 : receive, 3'h4 : wait."]
    #[inline(always)]
    pub fn state_ch4(&mut self) -> STATE_CH4_W {
        STATE_CH4_W { w: self }
    }
    #[doc = "Bits 12:21 - The current memory write address of channel4."]
    #[inline(always)]
    pub fn mem_raddr_ex_ch4(&mut self) -> MEM_RADDR_EX_CH4_W {
        MEM_RADDR_EX_CH4_W { w: self }
    }
    #[doc = "Bits 0:9 - The current memory read address of channel4."]
    #[inline(always)]
    pub fn mem_waddr_ex_ch4(&mut self) -> MEM_WADDR_EX_CH4_W {
        MEM_WADDR_EX_CH4_W { w: self }
    }
}
