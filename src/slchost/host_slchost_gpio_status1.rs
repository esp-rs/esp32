#[doc = "Reader of register HOST_SLCHOST_GPIO_STATUS1"]
pub type R = crate::R<u32, super::HOST_SLCHOST_GPIO_STATUS1>;
#[doc = "Writer for register HOST_SLCHOST_GPIO_STATUS1"]
pub type W = crate::W<u32, super::HOST_SLCHOST_GPIO_STATUS1>;
#[doc = "Register HOST_SLCHOST_GPIO_STATUS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_GPIO_STATUS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_GPIO_SDIO_INT1`"]
pub type HOST_GPIO_SDIO_INT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOST_GPIO_SDIO_INT1`"]
pub struct HOST_GPIO_SDIO_INT1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_GPIO_SDIO_INT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_gpio_sdio_int1(&self) -> HOST_GPIO_SDIO_INT1_R {
        HOST_GPIO_SDIO_INT1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_gpio_sdio_int1(&mut self) -> HOST_GPIO_SDIO_INT1_W {
        HOST_GPIO_SDIO_INT1_W { w: self }
    }
}
