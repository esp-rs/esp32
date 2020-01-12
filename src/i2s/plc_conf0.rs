#[doc = "Reader of register PLC_CONF0"]
pub type R = crate::R<u32, super::PLC_CONF0>;
#[doc = "Writer for register PLC_CONF0"]
pub type W = crate::W<u32, super::PLC_CONF0>;
#[doc = "Register PLC_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PLC_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `N_MIN_ERR`"]
pub type N_MIN_ERR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `N_MIN_ERR`"]
pub struct N_MIN_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> N_MIN_ERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Reader of field `PACK_LEN_8K`"]
pub type PACK_LEN_8K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACK_LEN_8K`"]
pub struct PACK_LEN_8K_W<'a> {
    w: &'a mut W,
}
impl<'a> PACK_LEN_8K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `MAX_SLIDE_SAMPLE`"]
pub type MAX_SLIDE_SAMPLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAX_SLIDE_SAMPLE`"]
pub struct MAX_SLIDE_SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_SLIDE_SAMPLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
#[doc = "Reader of field `SHIFT_RATE`"]
pub type SHIFT_RATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHIFT_RATE`"]
pub struct SHIFT_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_RATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `N_ERR_SEG`"]
pub type N_ERR_SEG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `N_ERR_SEG`"]
pub struct N_ERR_SEG_W<'a> {
    w: &'a mut W,
}
impl<'a> N_ERR_SEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `GOOD_PACK_MAX`"]
pub type GOOD_PACK_MAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GOOD_PACK_MAX`"]
pub struct GOOD_PACK_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> GOOD_PACK_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn n_min_err(&self) -> N_MIN_ERR_R {
        N_MIN_ERR_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn pack_len_8k(&self) -> PACK_LEN_8K_R {
        PACK_LEN_8K_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn max_slide_sample(&self) -> MAX_SLIDE_SAMPLE_R {
        MAX_SLIDE_SAMPLE_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn shift_rate(&self) -> SHIFT_RATE_R {
        SHIFT_RATE_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn n_err_seg(&self) -> N_ERR_SEG_R {
        N_ERR_SEG_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn good_pack_max(&self) -> GOOD_PACK_MAX_R {
        GOOD_PACK_MAX_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn n_min_err(&mut self) -> N_MIN_ERR_W {
        N_MIN_ERR_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn pack_len_8k(&mut self) -> PACK_LEN_8K_W {
        PACK_LEN_8K_W { w: self }
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn max_slide_sample(&mut self) -> MAX_SLIDE_SAMPLE_W {
        MAX_SLIDE_SAMPLE_W { w: self }
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn shift_rate(&mut self) -> SHIFT_RATE_W {
        SHIFT_RATE_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn n_err_seg(&mut self) -> N_ERR_SEG_W {
        N_ERR_SEG_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn good_pack_max(&mut self) -> GOOD_PACK_MAX_W {
        GOOD_PACK_MAX_W { w: self }
    }
}
