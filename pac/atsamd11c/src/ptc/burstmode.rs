#[doc = "Register `BURSTMODE` reader"]
pub type R = crate::R<BurstmodeSpec>;
#[doc = "Register `BURSTMODE` writer"]
pub type W = crate::W<BurstmodeSpec>;
#[doc = "Field `CTSLOWPOWEREN` reader - Enable clear to send low power mode"]
pub type CtslowpowerenR = crate::BitReader;
#[doc = "Field `CTSLOWPOWEREN` writer - Enable clear to send low power mode"]
pub type CtslowpowerenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURSTMODE` reader - Burst mode setting"]
pub type BurstmodeR = crate::FieldReader;
#[doc = "Field `BURSTMODE` writer - Burst mode setting"]
pub type BurstmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 2 - Enable clear to send low power mode"]
    #[inline(always)]
    pub fn ctslowpoweren(&self) -> CtslowpowerenR {
        CtslowpowerenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Burst mode setting"]
    #[inline(always)]
    pub fn burstmode(&self) -> BurstmodeR {
        BurstmodeR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 2 - Enable clear to send low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctslowpoweren(&mut self) -> CtslowpowerenW<BurstmodeSpec> {
        CtslowpowerenW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Burst mode setting"]
    #[inline(always)]
    #[must_use]
    pub fn burstmode(&mut self) -> BurstmodeW<BurstmodeSpec> {
        BurstmodeW::new(self, 4)
    }
}
#[doc = "Enable burst or clear to send low power mode\n\nYou can [`read`](crate::Reg::read) this register and get [`burstmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`burstmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BurstmodeSpec;
impl crate::RegisterSpec for BurstmodeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`burstmode::R`](R) reader structure"]
impl crate::Readable for BurstmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`burstmode::W`](W) writer structure"]
impl crate::Writable for BurstmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BURSTMODE to value 0"]
impl crate::Resettable for BurstmodeSpec {
    const RESET_VALUE: u8 = 0;
}
