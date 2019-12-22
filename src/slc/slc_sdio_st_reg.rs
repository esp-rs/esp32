#[doc = "Reader of register SLC_SDIO_ST_REG"]
pub type R = crate::R<u32, super::SLC_SDIO_ST_REG>;
#[doc = "Writer for register SLC_SDIO_ST_REG"]
pub type W = crate::W<u32, super::SLC_SDIO_ST_REG>;
#[doc = "Register SLC_SDIO_ST_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_SDIO_ST_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_FUNC2_ACC_STATE`"]
pub type SLC_FUNC2_ACC_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC_FUNC2_ACC_STATE`"]
pub struct SLC_FUNC2_ACC_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FUNC2_ACC_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `SLC_FUNC1_ACC_STATE`"]
pub type SLC_FUNC1_ACC_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC_FUNC1_ACC_STATE`"]
pub struct SLC_FUNC1_ACC_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FUNC1_ACC_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLC_BUS_ST`"]
pub type SLC_BUS_ST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC_BUS_ST`"]
pub struct SLC_BUS_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_BUS_ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `SLC_SDIO_WAKEUP`"]
pub type SLC_SDIO_WAKEUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SDIO_WAKEUP`"]
pub struct SLC_SDIO_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SDIO_WAKEUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SLC_FUNC_ST`"]
pub type SLC_FUNC_ST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC_FUNC_ST`"]
pub struct SLC_FUNC_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FUNC_ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SLC_CMD_ST`"]
pub type SLC_CMD_ST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC_CMD_ST`"]
pub struct SLC_CMD_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_CMD_ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn slc_func2_acc_state(&self) -> SLC_FUNC2_ACC_STATE_R {
        SLC_FUNC2_ACC_STATE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn slc_func1_acc_state(&self) -> SLC_FUNC1_ACC_STATE_R {
        SLC_FUNC1_ACC_STATE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn slc_bus_st(&self) -> SLC_BUS_ST_R {
        SLC_BUS_ST_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_sdio_wakeup(&self) -> SLC_SDIO_WAKEUP_R {
        SLC_SDIO_WAKEUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn slc_func_st(&self) -> SLC_FUNC_ST_R {
        SLC_FUNC_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn slc_cmd_st(&self) -> SLC_CMD_ST_R {
        SLC_CMD_ST_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn slc_func2_acc_state(&mut self) -> SLC_FUNC2_ACC_STATE_W {
        SLC_FUNC2_ACC_STATE_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn slc_func1_acc_state(&mut self) -> SLC_FUNC1_ACC_STATE_W {
        SLC_FUNC1_ACC_STATE_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn slc_bus_st(&mut self) -> SLC_BUS_ST_W {
        SLC_BUS_ST_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_sdio_wakeup(&mut self) -> SLC_SDIO_WAKEUP_W {
        SLC_SDIO_WAKEUP_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn slc_func_st(&mut self) -> SLC_FUNC_ST_W {
        SLC_FUNC_ST_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn slc_cmd_st(&mut self) -> SLC_CMD_ST_W {
        SLC_CMD_ST_W { w: self }
    }
}
