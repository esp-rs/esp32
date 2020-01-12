#[doc = "Reader of register OUT_LINK"]
pub type R = crate::R<u32, super::OUT_LINK>;
#[doc = "Writer for register OUT_LINK"]
pub type W = crate::W<u32, super::OUT_LINK>;
#[doc = "Register OUT_LINK `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_LINK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUTLINK_PARK`"]
pub type OUTLINK_PARK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTLINK_PARK`"]
pub struct OUTLINK_PARK_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_PARK_W<'a> {
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
#[doc = "Reader of field `OUTLINK_RESTART`"]
pub type OUTLINK_RESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTLINK_RESTART`"]
pub struct OUTLINK_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_RESTART_W<'a> {
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
#[doc = "Reader of field `OUTLINK_START`"]
pub type OUTLINK_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTLINK_START`"]
pub struct OUTLINK_START_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_START_W<'a> {
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
#[doc = "Reader of field `OUTLINK_STOP`"]
pub type OUTLINK_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTLINK_STOP`"]
pub struct OUTLINK_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_STOP_W<'a> {
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
#[doc = "Reader of field `OUTLINK_ADDR`"]
pub type OUTLINK_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OUTLINK_ADDR`"]
pub struct OUTLINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn outlink_park(&self) -> OUTLINK_PARK_R {
        OUTLINK_PARK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn outlink_restart(&self) -> OUTLINK_RESTART_R {
        OUTLINK_RESTART_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn outlink_start(&self) -> OUTLINK_START_R {
        OUTLINK_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn outlink_stop(&self) -> OUTLINK_STOP_R {
        OUTLINK_STOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn outlink_addr(&self) -> OUTLINK_ADDR_R {
        OUTLINK_ADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn outlink_park(&mut self) -> OUTLINK_PARK_W {
        OUTLINK_PARK_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W {
        OUTLINK_RESTART_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W {
        OUTLINK_START_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W {
        OUTLINK_STOP_W { w: self }
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn outlink_addr(&mut self) -> OUTLINK_ADDR_W {
        OUTLINK_ADDR_W { w: self }
    }
}