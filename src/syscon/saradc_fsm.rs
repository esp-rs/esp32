#[doc = "Reader of register SARADC_FSM"]
pub type R = crate::R<u32, super::SARADC_FSM>;
#[doc = "Writer for register SARADC_FSM"]
pub type W = crate::W<u32, super::SARADC_FSM>;
#[doc = "Register SARADC_FSM `reset()`'s with value 0"]
impl crate::ResetValue for super::SARADC_FSM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SARADC_SAMPLE_CYCLE`"]
pub type SARADC_SAMPLE_CYCLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SARADC_SAMPLE_CYCLE`"]
pub struct SARADC_SAMPLE_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAMPLE_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SARADC_START_WAIT`"]
pub type SARADC_START_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SARADC_START_WAIT`"]
pub struct SARADC_START_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_START_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SARADC_STANDBY_WAIT`"]
pub type SARADC_STANDBY_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SARADC_STANDBY_WAIT`"]
pub struct SARADC_STANDBY_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_STANDBY_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SARADC_RSTB_WAIT`"]
pub type SARADC_RSTB_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SARADC_RSTB_WAIT`"]
pub struct SARADC_RSTB_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_RSTB_WAIT_W<'a> {
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
    pub fn saradc_sample_cycle(&self) -> SARADC_SAMPLE_CYCLE_R {
        SARADC_SAMPLE_CYCLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn saradc_start_wait(&self) -> SARADC_START_WAIT_R {
        SARADC_START_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn saradc_standby_wait(&self) -> SARADC_STANDBY_WAIT_R {
        SARADC_STANDBY_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn saradc_rstb_wait(&self) -> SARADC_RSTB_WAIT_R {
        SARADC_RSTB_WAIT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn saradc_sample_cycle(&mut self) -> SARADC_SAMPLE_CYCLE_W {
        SARADC_SAMPLE_CYCLE_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn saradc_start_wait(&mut self) -> SARADC_START_WAIT_W {
        SARADC_START_WAIT_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn saradc_standby_wait(&mut self) -> SARADC_STANDBY_WAIT_W {
        SARADC_STANDBY_WAIT_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn saradc_rstb_wait(&mut self) -> SARADC_RSTB_WAIT_W {
        SARADC_RSTB_WAIT_W { w: self }
    }
}
