#[doc = "Reader of register SYSCON_SARADC_FSM_REG"]
pub type R = crate::R<u32, super::SYSCON_SARADC_FSM_REG>;
#[doc = "Writer for register SYSCON_SARADC_FSM_REG"]
pub type W = crate::W<u32, super::SYSCON_SARADC_FSM_REG>;
#[doc = "Register SYSCON_SARADC_FSM_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_SARADC_FSM_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_SARADC_SAMPLE_CYCLE`"]
pub type SYSCON_SARADC_SAMPLE_CYCLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_SARADC_SAMPLE_CYCLE`"]
pub struct SYSCON_SARADC_SAMPLE_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_SARADC_SAMPLE_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_SARADC_START_WAIT`"]
pub type SYSCON_SARADC_START_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_SARADC_START_WAIT`"]
pub struct SYSCON_SARADC_START_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_SARADC_START_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_SARADC_STANDBY_WAIT`"]
pub type SYSCON_SARADC_STANDBY_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_SARADC_STANDBY_WAIT`"]
pub struct SYSCON_SARADC_STANDBY_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_SARADC_STANDBY_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_SARADC_RSTB_WAIT`"]
pub type SYSCON_SARADC_RSTB_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_SARADC_RSTB_WAIT`"]
pub struct SYSCON_SARADC_RSTB_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_SARADC_RSTB_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn syscon_saradc_sample_cycle(&self) -> SYSCON_SARADC_SAMPLE_CYCLE_R {
        SYSCON_SARADC_SAMPLE_CYCLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn syscon_saradc_start_wait(&self) -> SYSCON_SARADC_START_WAIT_R {
        SYSCON_SARADC_START_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn syscon_saradc_standby_wait(&self) -> SYSCON_SARADC_STANDBY_WAIT_R {
        SYSCON_SARADC_STANDBY_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn syscon_saradc_rstb_wait(&self) -> SYSCON_SARADC_RSTB_WAIT_R {
        SYSCON_SARADC_RSTB_WAIT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn syscon_saradc_sample_cycle(&mut self) -> SYSCON_SARADC_SAMPLE_CYCLE_W {
        SYSCON_SARADC_SAMPLE_CYCLE_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn syscon_saradc_start_wait(&mut self) -> SYSCON_SARADC_START_WAIT_W {
        SYSCON_SARADC_START_WAIT_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn syscon_saradc_standby_wait(&mut self) -> SYSCON_SARADC_STANDBY_WAIT_W {
        SYSCON_SARADC_STANDBY_WAIT_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn syscon_saradc_rstb_wait(&mut self) -> SYSCON_SARADC_RSTB_WAIT_W {
        SYSCON_SARADC_RSTB_WAIT_W { w: self }
    }
}
