use crate::utils::{
    async_utils::wait,
    browser::{is_android, is_webkit},
    data::count_truthy,
    dom::selector_to_element,
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use wasm_bindgen::{JsCast, JsValue};
type Filters = std::collections::HashMap<String, Vec<String>>;

// cspell-checker: disable

/**
 * Only single element selector are supported (no operators like space, +, >, etc).
 * `embed` and `position: fixed;` will be considered as blocked anyway because it always has no
 * offsetParent. Avoid `iframe` and anything with `[src=]` because they produce excess HTTP
 * requests.
 *
 * The "inappropriate" selectors are obfuscated. See https://github.com/fingerprintjs/fingerprintjs/issues/734.
 * A function is used instead of a plain object to help tree-shaking.
 *
 * The function code is generated automatically. See docs/content_blockers.md to learn how to
 * make the list.
 */
pub fn get_filters() -> Filters {
    let from_b64 = |s: &str| String::from_utf8(STANDARD.decode(s).unwrap()).unwrap();

    let mut filters = Filters::new();

    filters.insert(
        "abpIndo".to_string(),
        vec![
            "#Iklan-Melayang".to_string(),
            "#Kolom-Iklan-728".to_string(),
            "#SidebarIklan-wrapper".to_string(),
            "[title=\"ALIENBOLA\" i]".to_string(),
            from_b64("I0JveC1CYW5uZXItYWRz"),
        ],
    );

    filters.insert(
        "abpvn".to_string(),
        vec![
            ".quangcao".to_string(),
            "#mobileCatfish".to_string(),
            from_b64("LnNsb3NlLWFkcw=="),
            "[id^=\"bn_bottom_fixed_\"]".to_string(),
            "#pmadv".to_string(),
        ],
    );

    filters.insert(
        "adBlockFinland".to_string(),
        vec![
            ".mainostila".to_string(),
            from_b64("LnNwb25zb3JpdA=="),
            ".ylamainos".to_string(),
            from_b64("YVtocmVmKj0iL2NsaWNrdGhyZ2guYXNwPyJd"),
            from_b64("YVtocmVmXj0iaHR0cHM6Ly9hcHAucmVhZHBlYWsuY29tL2FkcyJd"),
        ],
    );

    filters.insert(
        "adBlockPersian".to_string(),
        vec![
            "#navbar_notice_50".to_string(),
            ".kadr".to_string(),
            "TABLE[width=\"140px\"]".to_string(),
            "#divAgahi".to_string(),
            from_b64("YVtocmVmXj0iaHR0cDovL2cxLnYuZndtcm0ubmV0L2FkLyJd"),
        ],
    );

    filters.insert(
        "adBlockWarningRemoval".to_string(),
        vec![
            "#adblock-honeypot".to_string(),
            ".adblocker-root".to_string(),
            ".wp_adblock_detect".to_string(),
            from_b64("LmhlYWRlci1ibG9ja2VkLWFk"),
            from_b64("I2FkX2Jsb2NrZXI="),
        ],
    );

    filters.insert(
        "adGuardAnnoyances".to_string(),
        vec![
            ".hs-sosyal".to_string(),
            "#cookieconsentdiv".to_string(),
            "div[class^=\"app_gdpr\"]".to_string(),
            ".as-oil".to_string(),
            "[data-cypress=\"soft-push-notification-modal\"]".to_string(),
        ],
    );

    filters.insert(
        "adGuardBase".to_string(),
        vec![
            ".BetterJsPopOverlay".to_string(),
            from_b64("I2FkXzMwMFgyNTA="),
            from_b64("I2Jhbm5lcmZsb2F0MjI="),
            from_b64("I2NhbXBhaWduLWJhbm5lcg=="),
            from_b64("I0FkLUNvbnRlbnQ="),
        ],
    );

    filters.insert(
        "adGuardChinese".to_string(),
        vec![
            from_b64("LlppX2FkX2FfSA=="),
            from_b64("YVtocmVmKj0iLmh0aGJldDM0LmNvbSJd"),
            "#widget-quan".to_string(),
            from_b64("YVtocmVmKj0iLzg0OTkyMDIwLnh5eiJd"),
            from_b64("YVtocmVmKj0iLjE5NTZobC5jb20vIl0="),
        ],
    );

    filters.insert(
        "adGuardFrench".to_string(),
        vec![
            "#pavePub".to_string(),
            from_b64("LmFkLWRlc2t0b3AtcmVjdGFuZ2xl"),
            ".mobile_adhesion".to_string(),
            ".widgetadv".to_string(),
            from_b64("LmFkc19iYW4="),
        ],
    );

    filters.insert(
        "adGuardGerman".to_string(),
        vec!["aside[data-portal-id=\"leaderboard\"]".to_string()],
    );

    filters.insert(
        "adGuardJapanese".to_string(),
        vec![
            "#kauli_yad_1".to_string(),
            from_b64("YVtocmVmXj0iaHR0cDovL2FkMi50cmFmZmljZ2F0ZS5uZXQvIl0="),
            from_b64("Ll9wb3BJbl9pbmZpbml0ZV9hZA=="),
            from_b64("LmFkZ29vZ2xl"),
            from_b64("Ll9faXNib29zdFJldHVybkFk"),
        ],
    );

    filters.insert(
        "adGuardMobile".to_string(),
        vec![
            from_b64("YW1wLWF1dG8tYWRz"),
            from_b64("LmFtcF9hZA=="),
            "amp-embed[type=\"24smi\"]".to_string(),
            "#mgid_iframe1".to_string(),
            from_b64("I2FkX2ludmlld19hcmVh"),
        ],
    );

    filters.insert(
        "adGuardRussian".to_string(),
        vec![
            from_b64("YVtocmVmXj0iaHR0cHM6Ly9hZC5sZXRtZWFkcy5jb20vIl0="),
            from_b64("LnJlY2xhbWE="),
            "div[id^=\"smi2adblock\"]".to_string(),
            from_b64("ZGl2W2lkXj0iQWRGb3hfYmFubmVyXyJd"),
            "#psyduckpockeball".to_string(),
        ],
    );

    filters.insert(
        "adGuardSocial".to_string(),
        vec![
            from_b64("YVtocmVmXj0iLy93d3cuc3R1bWJsZXVwb24uY29tL3N1Ym1pdD91cmw9Il0="),
            from_b64("YVtocmVmXj0iLy90ZWxlZ3JhbS5tZS9zaGFyZS91cmw/Il0="),
            ".etsy-tweet".to_string(),
            "#inlineShare".to_string(),
            ".popup-social".to_string(),
        ],
    );

    filters.insert(
        "adGuardSpanishPortuguese".to_string(),
        vec![
            "#barraPublicidade".to_string(),
            "#Publicidade".to_string(),
            "#publiEspecial".to_string(),
            "#queTooltip".to_string(),
            ".cnt-publi".to_string(),
        ],
    );

    filters.insert(
        "adGuardTrackingProtection".to_string(),
        vec![
            "#qoo-counter".to_string(),
            from_b64("YVtocmVmXj0iaHR0cDovL2NsaWNrLmhvdGxvZy5ydS8iXQ=="),
            from_b64("YVtocmVmXj0iaHR0cDovL2hpdGNvdW50ZXIucnUvdG9wL3N0YXQucGhwIl0="),
            from_b64("YVtocmVmXj0iaHR0cDovL3RvcC5tYWlsLnJ1L2p1bXAiXQ=="),
            "#top100counter".to_string(),
        ],
    );

    filters.insert(
        "adGuardTurkish".to_string(),
        vec![
            "#backkapat".to_string(),
            from_b64("I3Jla2xhbWk="),
            from_b64("YVtocmVmXj0iaHR0cDovL2Fkc2Vydi5vbnRlay5jb20udHIvIl0="),
            from_b64("YVtocmVmXj0iaHR0cDovL2l6bGVuemkuY29tL2NhbXBhaWduLyJd"),
            from_b64("YVtocmVmXj0iaHR0cDovL3d3dy5pbnN0YWxsYWRzLm5ldC8iXQ=="),
        ],
    );

    filters.insert(
        "bulgarian".to_string(),
        vec![
            from_b64("dGQjZnJlZW5ldF90YWJsZV9hZHM="),
            "#ea_intext_div".to_string(),
            ".lapni-pop-over".to_string(),
            "#xenium_hot_offers".to_string(),
        ],
    );

    filters.insert(
        "easyList".to_string(),
        vec![
            ".yb-floorad".to_string(),
            from_b64("LndpZGdldF9wb19hZHNfd2lkZ2V0"),
            from_b64("LnRyYWZmaWNqdW5reS1hZA=="),
            ".textad_headline".to_string(),
            from_b64("LnNwb25zb3JlZC10ZXh0LWxpbmtz"),
        ],
    );

    filters.insert(
        "easyListChina".to_string(),
        vec![
            from_b64("LmFwcGd1aWRlLXdyYXBbb25jbGljayo9ImJjZWJvcy5jb20iXQ=="),
            from_b64("LmZyb250cGFnZUFkdk0="),
            "#taotaole".to_string(),
            "#aafoot.top_box".to_string(),
            ".cfa_popup".to_string(),
        ],
    );

    filters.insert(
        "easyListCookie".to_string(),
        vec![
            ".ezmob-footer".to_string(),
            ".cc-CookieWarning".to_string(),
            "[data-cookie-number]".to_string(),
            from_b64("LmF3LWNvb2tpZS1iYW5uZXI="),
            ".sygnal24-gdpr-modal-wrap".to_string(),
        ],
    );

    filters.insert(
        "easyListCzechSlovak".to_string(),
        vec![
            "#onlajny-stickers".to_string(),
            from_b64("I3Jla2xhbW5pLWJveA=="),
            from_b64("LnJla2xhbWEtbWVnYWJvYXJk"),
            ".sklik".to_string(),
            from_b64("W2lkXj0ic2tsaWtSZWtsYW1hIl0="),
        ],
    );

    filters.insert(
        "easyListDutch".to_string(),
        vec![
            from_b64("I2FkdmVydGVudGll"),
            from_b64("I3ZpcEFkbWFya3RCYW5uZXJCbG9jaw=="),
            ".adstekst".to_string(),
            from_b64("YVtocmVmXj0iaHR0cHM6Ly94bHR1YmUubmwvY2xpY2svIl0="),
            "#semilo-lrectangle".to_string(),
        ],
    );

    filters.insert(
        "easyListGermany".to_string(),
        vec![
            "#SSpotIMPopSlider".to_string(),
            from_b64("LnNwb25zb3JsaW5rZ3J1ZW4="),
            from_b64("I3dlcmJ1bmdza3k="),
            from_b64("I3Jla2xhbWUtcmVjaHRzLW1pdHRl"),
            from_b64("YVtocmVmXj0iaHR0cHM6Ly9iZDc0Mi5jb20vIl0="),
        ],
    );

    filters.insert(
        "easyListItaly".to_string(),
        vec![
            from_b64("LmJveF9hZHZfYW5udW5jaQ=="),
            ".sb-box-pubbliredazionale".to_string(),
            from_b64("YVtocmVmXj0iaHR0cDovL2FmZmlsaWF6aW9uaWFkcy5zbmFpLml0LyJd"),
            from_b64("YVtocmVmXj0iaHR0cHM6Ly9hZHNlcnZlci5odG1sLml0LyJd"),
            from_b64("YVtocmVmXj0iaHR0cHM6Ly9hZmZpbGlhemlvbmlhZHMuc25haS5pdC8iXQ=="),
        ],
    );

    filters.insert(
        "easyListLithuania".to_string(),
        vec![
            from_b64("LnJla2xhbW9zX3RhcnBhcw=="),
            from_b64("LnJla2xhbW9zX251b3JvZG9z"),
            from_b64("aW1nW2FsdD0iUmVrbGFtaW5pcyBza3lkZWxpcyJd"),
            from_b64("aW1nW2FsdD0iRGVkaWt1b3RpLmx0IHNlcnZlcmlhaSJd"),
            from_b64("aW1nW2FsdD0iSG9zdGluZ2FzIFNlcnZlcmlhaS5sdCJd"),
        ],
    );

    filters.insert(
        "estonian".to_string(),
        vec![from_b64("QVtocmVmKj0iaHR0cDovL3BheTRyZXN1bHRzMjQuZXUiXQ==")],
    );

    filters.insert(
        "fanboyAnnoyances".to_string(),
        vec![
            "#ac-lre-player".to_string(),
            ".navigate-to-top".to_string(),
            "#subscribe_popup".to_string(),
            ".newsletter_holder".to_string(),
            "#back-top".to_string(),
        ],
    );

    filters.insert(
        "fanboyAntiFacebook".to_string(),
        vec![".util-bar-module-firefly-visible".to_string()],
    );

    filters.insert(
        "fanboyEnhancedTrackers".to_string(),
        vec![
            ".open.pushModal".to_string(),
            "#issuem-leaky-paywall-articles-zero-remaining-nag".to_string(),
            "#sovrn_container".to_string(),
            "div[class$=\"-hide\"][zoompage-fontsize][style=\"display: block;\"]".to_string(),
            ".BlockNag__Card".to_string(),
        ],
    );

    filters.insert(
        "fanboySocial".to_string(),
        vec![
            "#FollowUs".to_string(),
            "#meteored_share".to_string(),
            "#social_follow".to_string(),
            ".article-sharer".to_string(),
            ".community__social-desc".to_string(),
        ],
    );

    filters.insert(
        "frellwitSwedish".to_string(),
        vec![
            from_b64("YVtocmVmKj0iY2FzaW5vcHJvLnNlIl1bdGFyZ2V0PSJfYmxhbmsiXQ=="),
            from_b64("YVtocmVmXj0iZG9rdG9yLXNlLm9uZWxpbmsubWUiXQ=="),
            "article.category-samarbete".to_string(),
            from_b64("ZGl2LmhvbGlkQWRz"),
            "ul.adsmodern".to_string(),
        ],
    );

    filters.insert(
        "greekAdBlock".to_string(),
        vec![
            from_b64("QVtocmVmKj0iYWRtYW4ub3RlbmV0LmdyL2NsaWNrPyJd"),
            from_b64("QVtocmVmKj0iaHR0cDovL2F4aWFiYW5uZXJzLmV4b2R1cy5nci8iXQ=="),
            from_b64("QVtocmVmKj0iaHR0cDovL2ludGVyYWN0aXZlLmZvcnRobmV0LmdyL2NsaWNrPyJd"),
            "DIV.agores300".to_string(),
            "TABLE.advright".to_string(),
        ],
    );

    filters.insert(
        "hungarian".to_string(),
        vec![
            "#cemp_doboz".to_string(),
            ".optimonk-iframe-container".to_string(),
            from_b64("LmFkX19tYWlu"),
            from_b64("W2NsYXNzKj0iR29vZ2xlQWRzIl0="),
            "#hirdetesek_box".to_string(),
        ],
    );

    filters.insert(
        "iDontCareAboutCookies".to_string(),
        vec![
            ".alert-info[data-block-track*=\"CookieNotice\"]".to_string(),
            ".ModuleTemplateCookieIndicator".to_string(),
            ".o--cookies--container".to_string(),
            "#cookies-policy-sticky".to_string(),
            "#stickyCookieBar".to_string(),
        ],
    );

    filters.insert(
        "icelandicAbp".to_string(),
        vec![from_b64(
            "QVtocmVmXj0iL2ZyYW1ld29yay9yZXNvdXJjZXMvZm9ybXMvYWRzLmFzcHgiXQ==",
        )],
    );

    filters.insert("latvian".to_string(), vec![
        from_b64("YVtocmVmPSJodHRwOi8vd3d3LnNhbGlkemluaS5sdi8iXVtzdHlsZT0iZGlzcGxheTogYmxvY2s7IHdpZHRoOiAxMjBweDsgaGVpZ2h0OiA0MHB4OyBvdmVyZmxvdzogaGlkZGVuOyBwb3NpdGlvbjogcmVsYXRpdmU7Il0="),
        from_b64("YVtocmVmPSJodHRwOi8vd3d3LnNhbGlkemluaS5sdi8iXVtzdHlsZT0iZGlzcGxheTogYmxvY2s7IHdpZHRoOiA4OHB4OyBoZWlnaHQ6IDMxcHg7IG92ZXJmbG93OiBoaWRkZW47IHBvc2l0aW9uOiByZWxhdGl2ZTsiXQ=="),
    ]);

    filters.insert(
        "listKr".to_string(),
        vec![
            from_b64("YVtocmVmKj0iLy9hZC5wbGFuYnBsdXMuY28ua3IvIl0="),
            from_b64("I2xpdmVyZUFkV3JhcHBlcg=="),
            from_b64("YVtocmVmXj0iLy9hZHYuaW1hZHJlcC5jby5rci8iXQ=="),
            from_b64("aW5zLmZhc3R2aWV3LWFk"),
            ".revenue_unit_item.dable".to_string(),
        ],
    );

    filters.insert(
        "listeAr".to_string(),
        vec![
            from_b64("LmdlbWluaUxCMUFk"),
            ".right-and-left-sponsers".to_string(),
            from_b64("YVtocmVmXj0iLmFmbGFtLmluZm8iXQ=="),
            from_b64("YVtocmVmXj0iYm9vcmFxLm9yZyJd"),
            from_b64("YVtocmVmXj0iZHViaXp6bGUuY29tL2FyLz91dG1fc291cmNlPSJd"),
        ],
    );

    filters.insert(
        "listeFr".to_string(),
        vec![
            from_b64("YVtocmVmXj0iaHR0cDovL3Byb21vLnZhZG9yLmNvbS8iXQ=="),
            from_b64("I2FkY29udGFpbmVyX3JlY2hlcmNoZQ=="),
            from_b64("YVtocmVmKj0id2Vib3JhbWEuZnIvZmNnaS1iaW4vIl0="),
            ".site-pub-interstitiel".to_string(),
            "div[id^=\"crt-\"][data-criteo-id]".to_string(),
        ],
    );

    filters.insert(
        "officialPolish".to_string(),
        vec![
            "#ceneo-placeholder-ceneo-12".to_string(),
            from_b64("W2hyZWZePSJodHRwczovL2FmZi5zZW5kaHViLnBsLyJd"),
            from_b64("YVtocmVmXj0iaHR0cDovL2Fkdm1hbmFnZXIudGVjaGZ1bi5wbC9yZWRpcmVjdC8iXQ=="),
            from_b64("YVtocmVmXj0iaHR0cDovL3d3dy50cml6ZXIucGwvP3V0bV9zb3VyY2UiXQ=="),
            from_b64("ZGl2I3NrYXBpZWNfYWQ="),
        ],
    );

    filters.insert(
        "ro".to_string(),
        vec![
            from_b64("YVtocmVmXj0iLy9hZmZ0cmsuYWx0ZXgucm8vQ291bnRlci9DbGljayJd"),
            from_b64("YVtocmVmXj0iaHR0cHM6Ly9ibGFja2ZyaWRheXNhbGVzLnJvL3Ryay9zaG9wLyJd"),
            from_b64("YVtocmVmXj0iaHR0cHM6Ly9ldmVudC4ycGVyZm9ybWFudC5jb20vZXZlbnRzL2NsaWNrIl0="),
            from_b64("YVtocmVmXj0iaHR0cHM6Ly9sLnByb2ZpdHNoYXJlLnJvLyJd"),
            "a[href^=\"/url/\"]".to_string(),
        ],
    );

    filters.insert(
        "ruAd".to_string(),
        vec![
            from_b64("YVtocmVmKj0iLy9mZWJyYXJlLnJ1LyJd"),
            from_b64("YVtocmVmKj0iLy91dGltZy5ydS8iXQ=="),
            from_b64("YVtocmVmKj0iOi8vY2hpa2lkaWtpLnJ1Il0="),
            "#pgeldiz".to_string(),
            ".yandex-rtb-block".to_string(),
        ],
    );

    filters.insert(
        "thaiAds".to_string(),
        vec![
            "a[href*=macau-uta-popup]".to_string(),
            from_b64("I2Fkcy1nb29nbGUtbWlkZGxlX3JlY3RhbmdsZS1ncm91cA=="),
            from_b64("LmFkczMwMHM="),
            ".bumq".to_string(),
            ".img-kosana".to_string(),
        ],
    );

    filters.insert(
        "webAnnoyancesUltralist".to_string(),
        vec![
            "#mod-social-share-2".to_string(),
            "#social-tools".to_string(),
            from_b64("LmN0cGwtZnVsbGJhbm5lcg=="),
            ".zergnet-recommend".to_string(),
            ".yt.btn-link.btn-md.btn".to_string(),
        ],
    );

    filters
}

/**
 * The order of the returned array means nothing (it's always sorted alphabetically).
 *
 * Notice that the source is slightly unstable.
 * Safari provides a 2-taps way to disable all content blockers on a page temporarily.
 * Also content blockers can be disabled permanently for a domain, but it requires 4 taps.
 * So empty array shouldn't be treated as "no blockers", it should be treated as "no signal".
 * If you are a website owner, don't make your visitors want to disable content blockers.
 */
pub async fn get_dom_blockers() -> Result<JsValue, JsValue> {
    if !is_applicable() {
        return Ok(JsValue::undefined());
    }

    let filters = get_filters();
    let filter_names: Vec<String> = filters.keys().cloned().collect();

    let mut all_selectors = Vec::new();
    for filter_name in &filter_names {
        if let Some(selectors) = filters.get(filter_name) {
            all_selectors.extend(selectors.clone());
        }
    }

    let blocked_selectors = get_blocked_selectors(&all_selectors).await;

    let mut active_blockers = Vec::new();
    for filter_name in &filter_names {
        if let Some(selectors) = filters.get(filter_name) {
            let blocked_count = count_truthy(
                &selectors
                    .iter()
                    .map(|s| blocked_selectors.contains_key(s))
                    .collect::<Vec<bool>>(),
            );
            if blocked_count > (selectors.len() as f64 * 0.6) as usize {
                active_blockers.push(filter_name.clone());
            }
        }
    }

    active_blockers.sort();

    let result = js_sys::Array::new();
    for blocker in active_blockers {
        result.push(&JsValue::from_str(&blocker));
    }

    Ok(JsValue::from(result))
}

pub fn is_applicable() -> bool {
    // Safari (desktop and mobile) and all Android browsers keep content blockers in both regular
    // and private mode
    is_webkit() || is_android()
}

pub async fn get_blocked_selectors(
    selectors: &[String],
) -> std::collections::HashMap<String, bool> {
    let document = web_sys::window().unwrap().document().unwrap();
    let root = document.create_element("div").unwrap();
    let mut elements = Vec::new();
    let mut blocked_selectors = std::collections::HashMap::new();

    force_show(&root);

    // First create all elements that can be blocked. If the DOM steps below are done in a single
    // cycle, browser will alternate tree modification and layout reading, that is very slow.
    for selector in selectors {
        let element = selector_to_element(selector);
        if element.tag_name() == "DIALOG" {
            if let Ok(dialog) = element.clone().dyn_into::<web_sys::HtmlDialogElement>() {
                let _ = dialog.show();
            }
        }
        // Protects from unwanted effects of `+` and `~` selectors of filters
        let holder = document.create_element("div").unwrap();
        force_show(&holder);
        holder.append_child(&element).unwrap();
        root.append_child(&holder).unwrap();
        elements.push(element);
    }

    // document.body can be null while the page is loading
    while document.body().is_none() {
        let _ = wait(50).await;
    }

    document.body().unwrap().append_child(&root).unwrap();

    // Then check which of the elements are blocked
    for (i, selector) in selectors.iter().enumerate() {
        let element = &elements[i];
        if let Ok(html_element) = element.clone().dyn_into::<web_sys::HtmlElement>() {
            if html_element.offset_parent().is_none() {
                blocked_selectors.insert(selector.clone(), true);
            }
        }
    }

    // Then remove the elements
    root.remove();

    blocked_selectors
}

fn force_show(element: &web_sys::Element) {
    if let Ok(html_element) = element.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = html_element.style().set_property("visibility", "hidden");
        let _ = html_element.style().set_property("display", "block");
    }
}
