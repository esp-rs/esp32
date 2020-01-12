#[doc = "Reader of register TAG_FO_CTRL"]
pub type R = crate::R<u32, super::TAG_FO_CTRL>;
#[doc = "Writer for register TAG_FO_CTRL"]
pub type W = crate::W<u32, super::TAG_FO_CTRL>;
#[doc = "Register TAG_FO_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TAG_FO_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_CACHE_TAG_PD`"]
pub type APP_CACHE_TAG_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CACHE_TAG_PD`"]
pub struct APP_CACHE_TAG_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_TAG_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `APP_CACHE_TAG_FORCE_ON`"]
pub type APP_CACHE_TAG_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CACHE_TAG_FORCE_ON`"]
pub struct APP_CACHE_TAG_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_TAG_FORCE_ON_W<'a> {
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
#[doc = "Reader of field `PRO_CACHE_TAG_PD`"]
pub type PRO_CACHE_TAG_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRO_CACHE_TAG_PD`"]
pub struct PRO_CACHE_TAG_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_TAG_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PRO_CACHE_TAG_FORCE_ON`"]
pub type PRO_CACHE_TAG_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRO_CACHE_TAG_FORCE_ON`"]
pub struct PRO_CACHE_TAG_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_TAG_FORCE_ON_W<'a> {
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
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn app_cache_tag_pd(&self) -> APP_CACHE_TAG_PD_R {
        APP_CACHE_TAG_PD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cache_tag_force_on(&self) -> APP_CACHE_TAG_FORCE_ON_R {
        APP_CACHE_TAG_FORCE_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_cache_tag_pd(&self) -> PRO_CACHE_TAG_PD_R {
        PRO_CACHE_TAG_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cache_tag_force_on(&self) -> PRO_CACHE_TAG_FORCE_ON_R {
        PRO_CACHE_TAG_FORCE_ON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn app_cache_tag_pd(&mut self) -> APP_CACHE_TAG_PD_W {
        APP_CACHE_TAG_PD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cache_tag_force_on(&mut self) -> APP_CACHE_TAG_FORCE_ON_W {
        APP_CACHE_TAG_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_cache_tag_pd(&mut self) -> PRO_CACHE_TAG_PD_W {
        PRO_CACHE_TAG_PD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cache_tag_force_on(&mut self) -> PRO_CACHE_TAG_FORCE_ON_W {
        PRO_CACHE_TAG_FORCE_ON_W { w: self }
    }
}
