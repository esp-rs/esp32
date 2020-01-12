#[doc = "Reader of register EXT_WAKEUP_CONF"]
pub type R = crate::R<u32, super::EXT_WAKEUP_CONF>;
#[doc = "Writer for register EXT_WAKEUP_CONF"]
pub type W = crate::W<u32, super::EXT_WAKEUP_CONF>;
#[doc = "Register EXT_WAKEUP_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT_WAKEUP_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXT_WAKEUP1_LV`"]
pub type EXT_WAKEUP1_LV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXT_WAKEUP1_LV`"]
pub struct EXT_WAKEUP1_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_WAKEUP1_LV_W<'a> {
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
#[doc = "Reader of field `EXT_WAKEUP0_LV`"]
pub type EXT_WAKEUP0_LV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXT_WAKEUP0_LV`"]
pub struct EXT_WAKEUP0_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_WAKEUP0_LV_W<'a> {
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
impl R {
    #[doc = "Bit 31 - 0: external wakeup at low level 1: external wakeup at high level"]
    #[inline(always)]
    pub fn ext_wakeup1_lv(&self) -> EXT_WAKEUP1_LV_R {
        EXT_WAKEUP1_LV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 0: external wakeup at low level 1: external wakeup at high level"]
    #[inline(always)]
    pub fn ext_wakeup0_lv(&self) -> EXT_WAKEUP0_LV_R {
        EXT_WAKEUP0_LV_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 0: external wakeup at low level 1: external wakeup at high level"]
    #[inline(always)]
    pub fn ext_wakeup1_lv(&mut self) -> EXT_WAKEUP1_LV_W {
        EXT_WAKEUP1_LV_W { w: self }
    }
    #[doc = "Bit 30 - 0: external wakeup at low level 1: external wakeup at high level"]
    #[inline(always)]
    pub fn ext_wakeup0_lv(&mut self) -> EXT_WAKEUP0_LV_W {
        EXT_WAKEUP0_LV_W { w: self }
    }
}
