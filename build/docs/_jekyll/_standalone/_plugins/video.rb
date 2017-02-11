module Jekyll
    class VideoBlockTag < Liquid::Block
        def initialize(tag_name, youtube_id, tokens)
            super
            @youtube_id = youtube_id.strip
        end

        def render(context)
            site = context.registers[:site]
            converter = site.getConverterImpl(Jekyll::Converters::Markdown)
            # Temporarily commented out TODO -- Markdown conversion seems to be causing some bugs
            #content = converter.convert(super.strip)
            content = super.strip 
            out = <<-eos
<a class="video" href="https://www.youtube.com/watch?v=#{@youtube_id}">
    #{content}
    <div class="video-modal" data-youtube-id="#{@youtube_id}">
        <div class="iframe-container"></div>
    </div>
</a>
            eos
        end
    end
end

Liquid::Template.register_tag('video', Jekyll::VideoBlockTag)
