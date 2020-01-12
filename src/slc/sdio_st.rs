#[doc = "Reader of register SDIO_ST"]
pub type R = crate::R<u32, super::SDIO_ST>;
#[doc = "Writer for register SDIO_ST"]
pub type W = crate::W<u32, super::SDIO_ST>;
#[doc = "Register SDIO_ST `reset()`'s with value 0"]
impl crate::ResetValue for super::SDIO_ST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FUNC2_ACC_STATE`"]
pub type FUNC2_ACC_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FUNC2_ACC_STATE`"]
pub struct FUNC2_ACC_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC2_ACC_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `FUNC1_ACC_STATE`"]
pub type FUNC1_ACC_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FUNC1_ACC_STATE`"]
pub struct FUNC1_ACC_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC1_ACC_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `BUS_ST`"]
pub type BUS_ST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUS_ST`"]
pub struct BUS_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `SDIO_WAKEUP`"]
pub type SDIO_WAKEUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_WAKEUP`"]
pub struct SDIO_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_WAKEUP_W<'a> {
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
#[doc = "Reader of field `FUNC_ST`"]
pub type FUNC_ST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FUNC_ST`"]
pub struct FUNC_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CMD_ST`"]
pub type CMD_ST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD_ST`"]
pub struct CMD_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_ST_W<'a> {
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
    pub fn func2_acc_state(&self) -> FUNC2_ACC_STATE_R {
        FUNC2_ACC_STATE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn func1_acc_state(&self) -> FUNC1_ACC_STATE_R {
        FUNC1_ACC_STATE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn bus_st(&self) -> BUS_ST_R {
        BUS_ST_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sdio_wakeup(&self) -> SDIO_WAKEUP_R {
        SDIO_WAKEUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn func_st(&self) -> FUNC_ST_R {
        FUNC_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cmd_st(&self) -> CMD_ST_R {
        CMD_ST_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn func2_acc_state(&mut self) -> FUNC2_ACC_STATE_W {
        FUNC2_ACC_STATE_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn func1_acc_state(&mut self) -> FUNC1_ACC_STATE_W {
        FUNC1_ACC_STATE_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn bus_st(&mut self) -> BUS_ST_W {
        BUS_ST_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sdio_wakeup(&mut self) -> SDIO_WAKEUP_W {
        SDIO_WAKEUP_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn func_st(&mut self) -> FUNC_ST_W {
        FUNC_ST_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cmd_st(&mut self) -> CMD_ST_W {
        CMD_ST_W { w: self }
    }
}
