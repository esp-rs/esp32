#[doc = "Reader of register APP_INTRUSION_CTRL"]
pub type R = crate::R<u32, super::APP_INTRUSION_CTRL>;
#[doc = "Writer for register APP_INTRUSION_CTRL"]
pub type W = crate::W<u32, super::APP_INTRUSION_CTRL>;
#[doc = "Register APP_INTRUSION_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_INTRUSION_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_INTRUSION_RECORD_RESET_N`"]
pub type APP_INTRUSION_RECORD_RESET_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_INTRUSION_RECORD_RESET_N`"]
pub struct APP_INTRUSION_RECORD_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_INTRUSION_RECORD_RESET_N_W<'a> {
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
    pub fn app_intrusion_record_reset_n(&self) -> APP_INTRUSION_RECORD_RESET_N_R {
        APP_INTRUSION_RECORD_RESET_N_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_intrusion_record_reset_n(&mut self) -> APP_INTRUSION_RECORD_RESET_N_W {
        APP_INTRUSION_RECORD_RESET_N_W { w: self }
    }
}
