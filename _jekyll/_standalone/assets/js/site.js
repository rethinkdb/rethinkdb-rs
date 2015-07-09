(function() {
  var rewrite_links, video_modals;

  $(function() {
    var $_sidebar, blog_sidebar_sticky;
    $('.menu-trigger a').click(function(event) {
      event.preventDefault();
      return $('body').toggleClass('pmr-open');
    });
    $_sidebar = $('.blog-sidebar ul');
    if ($_sidebar.length) {
      blog_sidebar_sticky = new Waypoint.Sticky({
        element: $('.blog-sidebar ul')
      });
    }
    $('.docs-nav h1, .mobile-doc-links h1').click(function(event) {
      return $(this).toggleClass('expand').next('ul').slideToggle('fast');
    });
    rewrite_links();
    return video_modals();
  });

  video_modals = function() {
    var dismiss_video;
    $('.video').click(function(event) {
      var $modal, attrs, iframe, opts, src, yt;
      event.preventDefault();
      $modal = $('.video-modal', this);
      yt = $modal.data('youtube-id');
      attrs = "width='560' height='315' frameborder='0' allowfullscreen";
      opts = "rel=0&autoplay=1&autohide=1";
      src = "src='//www.youtube.com/embed/" + yt + "?" + opts + "'";
      iframe = "<iframe " + attrs + " " + src + "></iframe>";
      $('.iframe-container', $modal).html(iframe);
      return $modal.fadeIn('fast');
    });
    dismiss_video = function() {
      var $modal;
      $modal = $('.video-modal:visible');
      return $modal.fadeOut('fast', function() {
        return $('.iframe-container').empty();
      });
    };
    $('.video-modal').on('click', function(event) {
      event.preventDefault();
      event.stopPropagation();
      return dismiss_video();
    });
    return $(document).keyup(function(event) {
      if (event.keyCode === 27) {
        return dismiss_video();
      }
    });
  };

  rewrite_links = function() {
    var href, i, lang, len, link, links_on_page, results, routes;
    routes = {
      '/api/': true,
      '/docs/changefeeds/': true,
      '/docs/cookbook/': true,
      '/docs/dates-and-times/': true,
      '/docs/geo-support/': true,
      '/docs/guide/': true,
      '/docs/nested-fields/': true,
      '/docs/publish-subscribe/': true,
      '/docs/rabbitmq/': true,
      '/docs/secondary-indexes/': true,
      '/docs/sql-to-reql/': true,
      '/docs/storing-binary/': true
    };
    lang = Cookies.get('lang');
    if (lang == null) {
      lang = 'javascript';
      Cookies.set('lang', lang, {
        path: '/'
      });
    } else if (/javascript/.test(document.location.pathname)) {
      if (lang !== 'javascript') {
        lang = 'javascript';
        Cookies.set('lang', lang, {
          path: '/'
        });
      }
    } else if (/python/.test(document.location.pathname)) {
      if (lang !== 'python') {
        lang = 'python';
        Cookies.set('lang', lang, {
          path: '/'
        });
      }
    } else if (/ruby/.test(document.location.pathname)) {
      if (lang !== 'ruby') {
        lang = 'ruby';
        Cookies.set('lang', lang, {
          path: '/'
        });
      }
    }
    links_on_page = $('a');
    results = [];
    for (i = 0, len = links_on_page.length; i < len; i++) {
      link = links_on_page[i];
      href = $(link).attr('href');
      if (routes[href] != null) {
        results.push($(link).attr('href', href + lang + '/'));
      } else {
        results.push(void 0);
      }
    }
    return results;
  };

}).call(this);
