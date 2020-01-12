#[doc = "Reader of register APP_BOOT_REMAP_CTRL"]
pub type R = crate::R<u32, super::APP_BOOT_REMAP_CTRL>;
#[doc = "Writer for register APP_BOOT_REMAP_CTRL"]
pub type W = crate::W<u32, super::APP_BOOT_REMAP_CTRL>;
#[doc = "Register APP_BOOT_REMAP_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_BOOT_REMAP_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_BOOT_REMAP`"]
pub type APP_BOOT_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_BOOT_REMAP`"]
pub struct APP_BOOT_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_BOOT_REMAP_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_boot_remap(&self) -> APP_BOOT_REMAP_R {
        APP_BOOT_REMAP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_boot_remap(&mut self) -> APP_BOOT_REMAP_W {
        APP_BOOT_REMAP_W { w: self }
    }
}
