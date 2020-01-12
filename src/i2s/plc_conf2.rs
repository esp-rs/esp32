#[doc = "Reader of register PLC_CONF2"]
pub type R = crate::R<u32, super::PLC_CONF2>;
#[doc = "Writer for register PLC_CONF2"]
pub type W = crate::W<u32, super::PLC_CONF2>;
#[doc = "Register PLC_CONF2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PLC_CONF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MIN_PERIOD`"]
pub type MIN_PERIOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN_PERIOD`"]
pub struct MIN_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
#[doc = "Reader of field `CVSD_SEG_MOD`"]
pub type CVSD_SEG_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CVSD_SEG_MOD`"]
pub struct CVSD_SEG_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSD_SEG_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn min_period(&self) -> MIN_PERIOD_R {
        MIN_PERIOD_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cvsd_seg_mod(&self) -> CVSD_SEG_MOD_R {
        CVSD_SEG_MOD_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn min_period(&mut self) -> MIN_PERIOD_W {
        MIN_PERIOD_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cvsd_seg_mod(&mut self) -> CVSD_SEG_MOD_W {
        CVSD_SEG_MOD_W { w: self }
    }
}
