(function() {
  var scroll_to;

  $(function() {
    var $api, $back_to_top, $back_to_top_sticky;
    $api = $('.api-sections');
    $('h2', $api).each(function(i, command_header) {
      var $header, $wrapper, command;
      $header = $(command_header);
      $wrapper = $("<article class='api-command'></article>");
      $header.nextUntil("h2").andSelf().wrapAll($wrapper);
      command = $('a', $header).attr('href').split('/').filter(function(el) {
        return el.length > 0;
      }).slice(-1)[0];
      return $("<a class='api-anchor' name='" + command + "'></a>").insertBefore($(command_header));
    });
    $('.api-nav.anchor-links .commands a').click(function(event) {
      var hash, scrolltop_offset;
      event.preventDefault();
      $('.commands a').removeClass('active');
      $(event.currentTarget).addClass('active');
      hash = $(event.currentTarget).attr('href').slice(1);
      scrolltop_offset = $("a.api-anchor[name='" + hash + "']").offset().top;
      return scroll_to(scrolltop_offset, function() {
        return window.location.hash = hash;
      });
    });
    $back_to_top = $('p.back-to-top');
    $back_to_top_sticky = new Waypoint.Sticky({
      element: $back_to_top
    });
    return $back_to_top.on('click', function(e) {
      e.preventDefault();
      return scroll_to(0);
    });
  });

  scroll_to = function(offset, callback) {
    return $('html, body').animate({
      scrollTop: offset
    }, 250, 'swing', callback);
  };

}).call(this);
