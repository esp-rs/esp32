#[doc = "Reader of register SW_CPU_STALL"]
pub type R = crate::R<u32, super::SW_CPU_STALL>;
#[doc = "Writer for register SW_CPU_STALL"]
pub type W = crate::W<u32, super::SW_CPU_STALL>;
#[doc = "Register SW_CPU_STALL `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_CPU_STALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_STALL_PROCPU_C1`"]
pub type SW_STALL_PROCPU_C1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_STALL_PROCPU_C1`"]
pub struct SW_STALL_PROCPU_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_STALL_PROCPU_C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `SW_STALL_APPCPU_C1`"]
pub type SW_STALL_APPCPU_C1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_STALL_APPCPU_C1`"]
pub struct SW_STALL_APPCPU_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_STALL_APPCPU_C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31 - {reg_sw_stall_procpu_c1\\[5:0\\] reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c1(&self) -> SW_STALL_PROCPU_C1_R {
        SW_STALL_PROCPU_C1_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c1(&self) -> SW_STALL_APPCPU_C1_R {
        SW_STALL_APPCPU_C1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31 - {reg_sw_stall_procpu_c1\\[5:0\\] reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c1(&mut self) -> SW_STALL_PROCPU_C1_W {
        SW_STALL_PROCPU_C1_W { w: self }
    }
    #[doc = "Bits 20:25 - {reg_sw_stall_appcpu_c1\\[5:0\\] reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c1(&mut self) -> SW_STALL_APPCPU_C1_W {
        SW_STALL_APPCPU_C1_W { w: self }
    }
}
