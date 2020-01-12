#[doc = "Reader of register APP_CPU_RECORD_CTRL"]
pub type R = crate::R<u32, super::APP_CPU_RECORD_CTRL>;
#[doc = "Writer for register APP_CPU_RECORD_CTRL"]
pub type W = crate::W<u32, super::APP_CPU_RECORD_CTRL>;
#[doc = "Register APP_CPU_RECORD_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_CPU_RECORD_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_CPU_PDEBUG_ENABLE`"]
pub type APP_CPU_PDEBUG_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CPU_PDEBUG_ENABLE`"]
pub struct APP_CPU_PDEBUG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CPU_PDEBUG_ENABLE_W<'a> {
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
#[doc = "Reader of field `APP_CPU_RECORD_DISABLE`"]
pub type APP_CPU_RECORD_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CPU_RECORD_DISABLE`"]
pub struct APP_CPU_RECORD_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CPU_RECORD_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `APP_CPU_RECORD_ENABLE`"]
pub type APP_CPU_RECORD_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CPU_RECORD_ENABLE`"]
pub struct APP_CPU_RECORD_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CPU_RECORD_ENABLE_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cpu_pdebug_enable(&self) -> APP_CPU_PDEBUG_ENABLE_R {
        APP_CPU_PDEBUG_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cpu_record_disable(&self) -> APP_CPU_RECORD_DISABLE_R {
        APP_CPU_RECORD_DISABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cpu_record_enable(&self) -> APP_CPU_RECORD_ENABLE_R {
        APP_CPU_RECORD_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cpu_pdebug_enable(&mut self) -> APP_CPU_PDEBUG_ENABLE_W {
        APP_CPU_PDEBUG_ENABLE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cpu_record_disable(&mut self) -> APP_CPU_RECORD_DISABLE_W {
        APP_CPU_RECORD_DISABLE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cpu_record_enable(&mut self) -> APP_CPU_RECORD_ENABLE_W {
        APP_CPU_RECORD_ENABLE_W { w: self }
    }
}