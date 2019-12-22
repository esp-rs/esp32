#[doc = "Reader of register DPORT_APP_DCACHE_DBUG3_REG"]
pub type R = crate::R<u32, super::DPORT_APP_DCACHE_DBUG3_REG>;
#[doc = "Writer for register DPORT_APP_DCACHE_DBUG3_REG"]
pub type W = crate::W<u32, super::DPORT_APP_DCACHE_DBUG3_REG>;
#[doc = "Register DPORT_APP_DCACHE_DBUG3_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_APP_DCACHE_DBUG3_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_APP_CACHE_IRAM0_PID_ERROR`"]
pub type DPORT_APP_CACHE_IRAM0_PID_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_CACHE_IRAM0_PID_ERROR`"]
pub struct DPORT_APP_CACHE_IRAM0_PID_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CACHE_IRAM0_PID_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_CPU_DISABLED_CACHE_IA`"]
pub type DPORT_APP_CPU_DISABLED_CACHE_IA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_APP_CPU_DISABLED_CACHE_IA`"]
pub struct DPORT_APP_CPU_DISABLED_CACHE_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CPU_DISABLED_CACHE_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 9)) | (((value as u32) & 0x3f) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dport_app_cache_iram0_pid_error(&self) -> DPORT_APP_CACHE_IRAM0_PID_ERROR_R {
        DPORT_APP_CACHE_IRAM0_PID_ERROR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 9:14"]
    #[inline(always)]
    pub fn dport_app_cpu_disabled_cache_ia(&self) -> DPORT_APP_CPU_DISABLED_CACHE_IA_R {
        DPORT_APP_CPU_DISABLED_CACHE_IA_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dport_app_cache_iram0_pid_error(&mut self) -> DPORT_APP_CACHE_IRAM0_PID_ERROR_W {
        DPORT_APP_CACHE_IRAM0_PID_ERROR_W { w: self }
    }
    #[doc = "Bits 9:14"]
    #[inline(always)]
    pub fn dport_app_cpu_disabled_cache_ia(&mut self) -> DPORT_APP_CPU_DISABLED_CACHE_IA_W {
        DPORT_APP_CPU_DISABLED_CACHE_IA_W { w: self }
    }
}
